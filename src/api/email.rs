use axum::{Json, extract::State};
use serde::Deserialize;
use ts_rs::TS;

use crate::{
    AppState, database,
    model::{Email, StatusResponse},
    utils::AppResult,
};

pub async fn route_list_emails<'src>(
    State(state): State<AppState<'src>>,
) -> AppResult<Json<Vec<Email>>> {
    Ok(Json(
        sqlx::query_as(r#"select * from email"#)
            .fetch_all(&state.db)
            .await?,
    ))
}

pub async fn route_create_email<'src>(
    State(state): State<AppState<'src>>,
    Json(body): Json<Email>,
) -> AppResult<Json<Email>> {
    let email = database::create_email(body, &state.db).await?;
    Ok(Json(email))
}

pub async fn route_update_email<'src>(
    State(state): State<AppState<'src>>,
    Json(body): Json<Email>,
) -> AppResult<Json<StatusResponse>> {
    sqlx::query(
        r#"update email set
            subject = $1,
            content = $2 where id = $3"#,
    )
    .bind(body.subject)
    .bind(body.content)
    .bind(body.id)
    .execute(&state.db)
    .await?;
    Ok(Json(StatusResponse::success()))
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
pub struct DeleteEmailBody {
    id: i32,
}
pub async fn route_delete_email<'src>(
    State(state): State<AppState<'src>>,
    Json(body): Json<DeleteEmailBody>,
) -> AppResult<Json<StatusResponse>> {
    sqlx::query(r#"delete from email where id = $1"#)
        .bind(body.id)
        .execute(&state.db)
        .await?;
    Ok(Json(StatusResponse::success()))
}
