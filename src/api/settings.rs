use axum::{Json, extract::State};

use crate::{
    AppState,
    model::{Settings, StatusResponse},
    utils::AppResult,
};

pub async fn route_get_settings<'src>(
    State(state): State<AppState<'src>>,
) -> AppResult<Json<Settings>> {
    Ok(Json(
        sqlx::query_as(r#"select * from settings where id = 'settings'"#)
            .fetch_one(&state.db)
            .await?,
    ))
}

pub async fn route_update_settings<'src>(
    State(state): State<AppState<'src>>,
    Json(body): Json<Settings>,
) -> AppResult<Json<StatusResponse>> {
    sqlx::query(
        r#"update settings set
            cleverreach_client_id = $1,
            cleverreach_secret = $2,
            h_captcha_secret = $3,
            re_captcha_secret = $4,
            captcha_sitekey = $5
        where id = 'settings'
        "#,
    )
    .bind(&body.cleverreach_client_id)
    .bind(&body.cleverreach_secret)
    .bind(body.h_captcha_secret)
    .bind(body.re_captcha_secret)
    .bind(body.captcha_sitekey)
    .execute(&state.db)
    .await?;
    let mut cleverreach = state.cleverreach.write().await;
    cleverreach.set_client_credentials(body.cleverreach_client_id, body.cleverreach_secret);
    Ok(Json(StatusResponse::success()))
}
