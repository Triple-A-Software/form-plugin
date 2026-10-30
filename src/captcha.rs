#[cfg(feature = "seed")]
use fake::Dummy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::Type;
use ts_rs::TS;

use crate::{model::Settings, utils::ErrorReport};

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
    Turnstile,
}

#[derive(Deserialize)]
pub struct CaptchaResponse {
    pub success: bool,
    #[serde(rename = "error-codes")]
    pub error_codes: Option<Vec<String>>,
}

impl Captcha<'_> {
    /// returns None if there is no secret configured, or self.body doesn't have a captcha
    /// response, or there was an error when sending the request
    pub async fn handle(&self, settings: Settings) -> Option<CaptchaResponse> {
        let secret = settings.get_captcha_secret(self.r#type);
        let url = match self.r#type {
            CaptchaType::HCaptcha => "https://api.hcaptcha.com/siteverify",
            CaptchaType::ReCaptcha => "https://www.google.com/recaptcha/api/siteverify",
            CaptchaType::Turnstile => "https://challenges.cloudflare.com/turnstile/v0/siteverify",
        };
        let response = match self.r#type {
            CaptchaType::HCaptcha => self.body.get("h-captcha-response"),
            CaptchaType::ReCaptcha => self.body.get("g-recaptcha-response"),
            CaptchaType::Turnstile => self.body.get("cf-turnstile-response"),
        };
        if let (Some(Value::String(response)), Some(secret)) = (response, secret) {
            // `.form(...)` url-encodes the body and sets the correct Content-Type and
            // Content-Length. We must NOT set Content-Length manually: a hand-computed length of
            // the un-encoded string is too short once the token contains characters that get
            // percent-encoded (common for reCaptcha tokens), which truncates the request body and
            // makes siteverify reject the token.
            reqwest::Client::new()
                .post(url)
                .form(&[("secret", secret), ("response", response)])
                .header("Accept", "application/json")
                .send()
                .await
                .report_err(file!(), line!())
                .ok()?
                .json::<CaptchaResponse>()
                .await
                .report_err(file!(), line!())
                .ok()
        } else {
            None
        }
    }
}
