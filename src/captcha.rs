#[cfg(feature = "seed")]
use fake::Dummy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::Type;
use ts_rs::TS;

use crate::model::Settings;

pub struct Captcha<'a> {
    pub body: &'a Value,
    pub r#type: &'a CaptchaType,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, TS, Type)]
#[cfg_attr(feature = "seed", derive(Dummy))]
#[ts(export, export_to = "index.ts")]
#[sqlx(type_name = "text", rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum CaptchaType {
    HCaptcha,
    ReCaptcha,
}

#[derive(Deserialize)]
pub struct CaptchaResponse {
    pub success: bool,
}

impl Captcha<'_> {
    /// returns None if there is no secret configured, or self.body doesn't have a captcha
    /// response, or there was an error when sending the request
    pub async fn handle(&self, settings: Settings) -> Option<CaptchaResponse> {
        let secret = settings.get_captcha_secret(self.r#type);
        let url = match self.r#type {
            CaptchaType::HCaptcha => "https://api.hcaptcha.com/siteverify",
            CaptchaType::ReCaptcha => "https://www.google.com/recaptcha/api/siteverify",
        };
        let response = match self.r#type {
            CaptchaType::HCaptcha => self.body.get("g-recaptcha-response"),
            CaptchaType::ReCaptcha => self.body.get("h-captcha-response"),
        };
        if let (Some(Value::String(response)), Some(secret)) = (response, secret) {
            reqwest::Client::new()
                .post(url)
                .body(format!("secret={secret}&response={response}"))
                .header("Content-type", "application/x-www-form-urlencoded")
                .send()
                .await
                .ok()?
                .json::<CaptchaResponse>()
                .await
                .ok()
        } else {
            None
        }
    }
}
