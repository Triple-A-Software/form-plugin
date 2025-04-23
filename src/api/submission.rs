use axum::{Json, Router, extract::State, routing::get};
use serde::Deserialize;
use ts_rs::TS;

use crate::{
    AppState,
    model::{StatusResponse, Submission},
};

pub fn submission_router() -> Router<AppState> {
    Router::new().route(
        "/api/submissions",
        get(list_submissions).delete(delete_submission),
    )
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
