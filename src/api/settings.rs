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
            cleverreach_secret = $2
        where id = 'settings'
        "#,
    )
    .bind(body.cleverreach_client_id)
    .bind(body.cleverreach_secret)
    .execute(&state.db)
    .await
    .unwrap();
    Json(StatusResponse::success())
}
