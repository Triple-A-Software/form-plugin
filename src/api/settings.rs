use axum::{Json, Router, extract::State, routing::get};

use crate::{
    AppState,
    model::{Settings, StatusResponse},
};

pub fn settings_router() -> Router<AppState> {
    Router::new().route("/api/settings", get(get_settings).put(update_settings))
}

async fn get_settings(State(state): State<AppState>) -> Json<Settings> {
    Json(
        sqlx::query_as(r#"select * from settings where id = 'settings'"#)
            .fetch_one(&state.db)
            .await
            .unwrap(),
    )
}

async fn update_settings(
    State(state): State<AppState>,
    Json(body): Json<Settings>,
) -> Json<StatusResponse> {
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
    .await
    .unwrap();
    let mut cleverreach = state.cleverreach.write().await;
    cleverreach.set_client_credentials(body.cleverreach_client_id, body.cleverreach_secret);
    Json(StatusResponse::success())
}
