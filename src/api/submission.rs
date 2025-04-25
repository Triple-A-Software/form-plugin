use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::Deserialize;
use ts_rs::TS;

use crate::{
    AppState, extract_field_as_string,
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

async fn list_submissions(State(state): State<AppState>) -> Json<Vec<Submission>> {
    Json(
        sqlx::query_as(r#"select * from submission"#)
            .fetch_all(&state.db)
            .await
            .unwrap(),
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
