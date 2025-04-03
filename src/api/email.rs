use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use serde::Deserialize;
use ts_rs::TS;

use crate::{
    AppState,
    model::{Email, StatusResponse},
};

pub fn email_router() -> Router<AppState> {
    Router::new().route(
        "/api/emails",
        get(list_emails)
            .post(create_email)
            .put(update_email)
            .delete(delete_email),
    )
}

async fn list_emails(State(state): State<AppState>) -> Json<Vec<Email>> {
    Json(
        sqlx::query_as(r#"select * from email"#)
            .fetch_all(&state.db)
            .await
            .unwrap(),
    )
}

async fn create_email(
    State(state): State<AppState>,
    Json(body): Json<Email>,
) -> Json<StatusResponse> {
    sqlx::query(
        r#"insert into email (
            subject,
            content
        ) values ($1, $2)"#,
    )
    .bind(body.subject)
    .bind(body.content)
    .execute(&state.db)
    .await
    .unwrap();
    Json(StatusResponse::success())
}

async fn update_email(
    State(state): State<AppState>,
    Json(body): Json<Email>,
) -> Json<StatusResponse> {
    sqlx::query(
        r#"update email set
            name = $1,
            subject = $2,
            content = $3 where id = $4"#,
    )
    .bind(body.subject)
    .bind(body.content)
    .bind(body.id)
    .execute(&state.db)
    .await
    .unwrap();
    Json(StatusResponse::success())
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
struct DeleteEmailQuery {
    id: i32,
}
async fn delete_email(
    State(state): State<AppState>,
    Query(query): Query<DeleteEmailQuery>,
) -> Json<StatusResponse> {
    sqlx::query(r#"delete from email where id = $1"#)
        .bind(query.id)
        .execute(&state.db)
        .await
        .unwrap();
    Json(StatusResponse::success())
}
