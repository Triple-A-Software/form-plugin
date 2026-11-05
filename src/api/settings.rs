use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    AppState, clamav,
    cleverreach::Cleverreach,
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
            turnstile_secret = $5,
            captcha_provider = $6,
            captcha_sitekey = $7,
            clamav_address = $8
        where id = 'settings'
        "#,
    )
    .bind(&body.cleverreach_client_id)
    .bind(&body.cleverreach_secret)
    .bind(body.h_captcha_secret)
    .bind(body.re_captcha_secret)
    .bind(body.turnstile_secret)
    .bind(body.captcha_provider)
    .bind(body.captcha_sitekey)
    .bind(body.clamav_address)
    .execute(&state.db)
    .await?;
    let mut cleverreach = state.cleverreach.write().await;
    cleverreach.set_client_credentials(body.cleverreach_client_id, body.cleverreach_secret);
    Ok(Json(StatusResponse::success()))
}

/// tests the values entered in the settings form, before they are saved
#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
#[serde(tag = "type")]
pub enum TestConnectionBody {
    Clamav { address: String },
    Cleverreach { client_id: String, client_secret: String },
}
#[derive(Serialize, TS)]
#[ts(export, export_to = "index.ts")]
pub struct TestConnectionResponse {
    ok: bool,
    error: Option<String>,
}
pub async fn route_test_connection(
    Json(body): Json<TestConnectionBody>,
) -> AppResult<Json<TestConnectionResponse>> {
    let result = match body {
        TestConnectionBody::Clamav { address } => {
            clamav::ping(address.trim()).await.map_err(|e| e.to_string())
        }
        TestConnectionBody::Cleverreach {
            client_id,
            client_secret,
        } => Cleverreach::test_credentials(client_id.trim(), client_secret.trim())
            .await
            .map_err(|e| e.to_string()),
    };
    Ok(Json(TestConnectionResponse {
        ok: result.is_ok(),
        error: result.err(),
    }))
}
