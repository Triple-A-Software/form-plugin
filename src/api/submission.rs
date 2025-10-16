use std::collections::HashMap;

use axum::{
    Json, Router,
    extract::{Query, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
    routing::{get, post},
};
use reqwest::header;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;

use crate::{
    AppState, convert_to_csv,
    date::JavascriptDate,
    extract_field_as_string,
    model::{Form, StatusResponse, Submission},
    send_email,
};

pub fn submission_router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/submissions",
            get(list_submissions).delete(delete_submission),
        )
        .route(
            "/api/submissions/send-email",
            post(send_email_to_submission),
        )
        .route("/api/submissions/count", get(count_submissions))
        .route("/api/submissions/recent", get(recent_submissions))
        .route("/api/submissions/stats", get(submission_stats))
        .route("/api/submissions/export", get(export_submissions))
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "index.ts")]
struct SubmissionStats {
    total: i32,
    #[ts(type = "Array<{ date: string } & Record<string, number>>")]
    graph: Vec<ApiGraphValue>,
}
#[derive(Serialize, FromRow)]
struct ApiGraphValue {
    date: JavascriptDate,
    #[serde(flatten)]
    values: HashMap<String, i32>,
}
#[derive(FromRow, Debug)]
struct GraphValue {
    date: JavascriptDate,
    form_id: i32,
    value: i64,
}
async fn submission_stats(State(state): State<AppState>) -> Json<SubmissionStats> {
    let graph: Vec<GraphValue> = sqlx::query_as(
        r#"
        select date_trunc('day', "timestamp") as date, count(timestamp) as value, form_id 
        from "submission"
        group by date, form_id order by date
        "#,
    )
    .fetch_all(&state.db)
    .await
    .unwrap();
    let total: i64 = sqlx::query_scalar(
        r#"
            select count(*) from "submission"
        "#,
    )
    .fetch_one(&state.db)
    .await
    .unwrap();
    let graph = graph
        .into_iter()
        .fold(Vec::new(), |mut acc: Vec<ApiGraphValue>, g: GraphValue| {
            if let Some(value) = acc.iter_mut().find(|a| a.date == g.date) {
                value.values.insert(g.form_id.to_string(), g.value as i32);
            } else {
                acc.push(ApiGraphValue {
                    date: g.date,
                    values: HashMap::from([(g.form_id.to_string(), g.value as i32)]),
                });
            }
            acc
        });
    Json(SubmissionStats {
        graph,
        total: total as i32,
    })
}

async fn recent_submissions(State(state): State<AppState>) -> Json<Vec<Submission>> {
    let submissions: Vec<Submission> =
        sqlx::query_as(r#"select * from submission order by timestamp desc limit 10"#)
            .fetch_all(&state.db)
            .await
            .unwrap();
    Json(submissions)
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
struct SendEmailToSubmissionBody {
    email_id: i32,
    submission_id: i32,
    email_field_name: Option<String>,
}
async fn send_email_to_submission(
    State(state): State<AppState>,
    Json(body): Json<SendEmailToSubmissionBody>,
) -> Json<StatusResponse> {
    let submission: Submission = sqlx::query_as(r#"select * from submission where id = $1"#)
        .bind(body.submission_id)
        .fetch_one(&state.db)
        .await
        .unwrap();
    let form: Form = sqlx::query_as(r#"select * from form where id = $1"#)
        .bind(submission.form_id)
        .fetch_one(&state.db)
        .await
        .unwrap();
    let email = extract_field_as_string(
        &submission.data,
        Some(body.email_field_name.as_deref().unwrap_or("email")),
    );
    if let Some(email) = email {
        send_email(&state.db, body.email_id, &form, &submission.data, &email).await;
        Json(StatusResponse::success())
    } else {
        Json(StatusResponse::fail())
    }
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
struct ListSubmissionsQuery {
    form_id: Option<i32>,
    page: i32,
    items_per_page: i32,
}
async fn list_submissions(
    State(state): State<AppState>,
    Query(query): Query<ListSubmissionsQuery>,
) -> Json<Vec<Submission>> {
    if let Some(form_id) = query.form_id {
        Json(
            sqlx::query_as(r#"select * from submission where form_id = $1 order by timestamp desc limit $2 offset $3"#)
                .bind(form_id)
                .bind(query.items_per_page)
                .bind(query.page * query.items_per_page)
                .fetch_all(&state.db)
                .await
                .unwrap(),
        )
    } else {
        Json(
            sqlx::query_as(
                r#"select * from submission order by timestamp desc limit $1 offset $2"#,
            )
            .bind(query.items_per_page)
            .bind(query.page * query.items_per_page)
            .fetch_all(&state.db)
            .await
            .unwrap(),
        )
    }
}

async fn count_submissions(
    State(state): State<AppState>,
    Query(query): Query<ListSubmissionsQuery>,
) -> Json<i64> {
    if let Some(form_id) = query.form_id {
        Json(
            sqlx::query_scalar(r#"select count(*) from submission where form_id = $1"#)
                .bind(form_id)
                .fetch_one(&state.db)
                .await
                .unwrap(),
        )
    } else {
        Json(
            sqlx::query_scalar(r#"select count(*) from submission"#)
                .fetch_one(&state.db)
                .await
                .unwrap(),
        )
    }
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
struct ExportSubmissionsQuery {
    form_id: Option<i32>,
}

async fn export_submissions(
    State(state): State<AppState>,
    Query(query): Query<ExportSubmissionsQuery>,
) -> impl IntoResponse {
    let submissions: Vec<Submission> = if let Some(form_id) = query.form_id {
        sqlx::query_as(r#"select * from submission where form_id = $1 order by timestamp asc"#)
            .bind(form_id)
            .fetch_all(&state.db)
            .await
            .unwrap()
    } else {
        sqlx::query_as(r#"select * from submission order by timestamp asc"#)
            .fetch_all(&state.db)
            .await
            .unwrap()
    };
    let mut headers = HeaderMap::new();
    let date = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S");
    headers.insert("Content-type", HeaderValue::from_static("text/csv"));
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(
            format!("attachment; filename=\"submissions_export_{date}.csv\"").as_str(),
        )
        .unwrap(),
    );
    (
        headers,
        convert_to_csv(
            &submissions
                .into_iter()
                .map(|s| serde_json::to_value(s).unwrap())
                .collect::<Vec<_>>(),
        ),
    )
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
struct DeleteSubmissionBody {
    id: i32,
}
async fn delete_submission(
    State(state): State<AppState>,
    Json(body): Json<DeleteSubmissionBody>,
) -> Json<StatusResponse> {
    sqlx::query(r#"delete from submission where id = $1"#)
        .bind(body.id)
        .execute(&state.db)
        .await
        .unwrap();
    Json(StatusResponse::success())
}
