use axum::{
    Json, Router,
    extract::{Query, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use ts_rs::TS;

use crate::{
    AppState, convert_to_csv, extract_field_as_string,
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
        .route("/api/submissions/export", get(export_submissions))
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
                .bind(query.page)
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
            .bind(query.page)
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

async fn export_submissions(
    State(state): State<AppState>,
    Query(query): Query<ListSubmissionsQuery>,
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
    headers.insert(
        "Content-disposition",
        HeaderValue::from_str(
            r#"Attachment;filename="submissions_export_${new Date().toDateString()}.csv"#,
        )
        .unwrap(),
    );
    headers.insert("Content-type", HeaderValue::from_static("text/csv"));
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
