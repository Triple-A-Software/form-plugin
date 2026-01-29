use axum::{Json, extract::State};
use serde::Deserialize;
use ts_rs::TS;

use crate::{
    AppState, database,
    model::{Email, StatusResponse},
};

pub async fn route_list_emails<'src>(State(state): State<AppState<'src>>) -> Json<Vec<Email>> {
    Json(
        sqlx::query_as(r#"select * from email"#)
            .fetch_all(&state.db)
            .await
            .unwrap(),
    )
}

pub async fn route_create_email<'src>(
    State(state): State<AppState<'src>>,
    Json(body): Json<Email>,
) -> Json<Email> {
    let email = database::create_email(body, &state.db).await.unwrap();
    Json(email)
}

pub async fn route_update_email<'src>(
    State(state): State<AppState<'src>>,
    Json(body): Json<Email>,
) -> Json<StatusResponse> {
    sqlx::query(
        r#"update email set
            subject = $1,
            content = $2 where id = $3"#,
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
pub struct DeleteEmailBody {
    id: i32,
}
pub async fn route_delete_email<'src>(
    State(state): State<AppState<'src>>,
    Json(body): Json<DeleteEmailBody>,
) -> Json<StatusResponse> {
    sqlx::query(r#"delete from email where id = $1"#)
        .bind(body.id)
        .execute(&state.db)
        .await
        .unwrap();
    Json(StatusResponse::success())
}
