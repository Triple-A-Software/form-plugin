use std::{collections::HashMap, env, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::*, types::Json};
use tera::Context;
use ts_rs::TS;

use crate::date::JavascriptDate;

#[derive(Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct StatusResponse {
    success: bool,
}

impl StatusResponse {
    pub fn success() -> Self {
        Self { success: true }
    }
}

#[derive(Serialize, TS, Debug, Clone)]
#[ts(export, export_to = "index.ts")]
pub struct ApiError {
    error_message: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS, FromRow)]
#[ts(export, export_to = "index.ts")]
pub struct Submission {
    id: i32,
    form_id: i32,
    timestamp: JavascriptDate,
    #[ts(as = "Value")]
    data: Json<Value>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS, FromRow)]
#[ts(export, export_to = "index.ts")]
pub struct Form {
    pub id: i32,
    pub name: String,
    pub send_confirmation_email: bool,
    pub confirmation_email_id: Option<i32>,
    pub confirmation_email_recipient_field_name: Option<String>,
    pub send_submission_as_email_enabled: bool,
    pub send_submission_as_email: Option<String>,
    pub submissions_email_id: Option<i32>,
    pub activate_cleverreach: bool,
    #[ts(as = "CleverreachFormCondition")]
    pub cleverreach_condition: Json<CleverreachFormCondition>,
    pub cleverreach_group_id: Option<String>,
    pub cleverreach_email_field_name: Option<String>,
    pub cleverreach_firstname_field_name: Option<String>,
    pub cleverreach_lastname_field_name: Option<String>,
    pub cleverreach_salutation: Option<String>,
    #[ts(as = "Vec<Property>")]
    pub props: Json<Vec<Property>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(tag = "type")]
#[ts(export, export_to = "index.ts")]
pub enum CleverreachFormCondition {
    OnSubmit,
    WhenFieldIsSet(String),
}

#[derive(Deserialize, Serialize, Clone, Debug, TS, FromRow)]
#[ts(export, export_to = "index.ts")]
pub struct Email {
    pub id: i32,
    pub subject: String,
    /// html code for the email body
    pub content: String,
}

impl Email {
    pub async fn send(&self, receiver: &str, render_context: Context) -> anyhow::Result<()> {
        let internal_backend_port = env::var("INTERNAL_BACKEND_PORT")?;
        reqwest::Client::new()
            .post(format!("localhost:{}/email/send", internal_backend_port))
            .body(serde_json::to_string(&EmailSendRequest {
                receiver,
                subject: self.subject.clone(),
                content: tera::Tera::one_off(&self.content, &render_context, true).unwrap(),
            })?)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct EmailSendRequest<'a> {
    pub receiver: &'a str,
    pub subject: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[ts(export, export_to = "index.ts")]
pub enum Property {
    Textarea(TextareaProperty),
    Text(TextProperty),
    Checkbox(CheckboxProperty),
    Number(NumberProperty),
    Select(SelectProperty),
    Range(RangeProperty),
    Buttongroup(ButtongroupProperty),
    Button(ButtonProperty),
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct PropertyCommon {
    #[ts(type = "Record<string, string>")]
    hint: HashMap<String, String>,
    #[ts(type = "Record<string, string>")]
    label: HashMap<String, String>,
    default_value: Option<Value>,
    class: String,
    name: String,
    required: bool,
    color: TailwindColor,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct ButtonProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    button_type: ButtonType,
    variant: Variant,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = "index.ts")]
pub enum ButtonType {
    Button,
    Submit,
    Reset,
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonType::Button => f.write_str("button"),
            ButtonType::Submit => f.write_str("submit"),
            ButtonType::Reset => f.write_str("reset"),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct CheckboxProperty {
    #[serde(flatten)]
    common: PropertyCommon,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct TextareaProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    max_length: Option<i32>,
    #[ts(type = "Record<string, string>")]
    placeholder: HashMap<String, String>,
    rows: Option<i32>,
    columns: Option<i32>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct TextProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    max: Option<i32>,
    #[ts(type = "Record<string, string>")]
    placeholder: HashMap<String, String>,
    input_type: TextInputType,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = "index.ts")]
pub enum TextInputType {
    Text,
    Email,
    Password,
    Color,
    Tel,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct NumberProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    #[ts(type = "Record<string, string>")]
    placeholder: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct SelectProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    multiple: Option<bool>,
    data_type: SelectDataType,
    options: Vec<SelectOption>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "index.ts")]
pub struct SelectOption {
    #[ts(type = "Record<string, string>")]
    label: HashMap<String, String>,
    value: SelectData,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "kebab-case", untagged)]
#[ts(export, export_to = "index.ts")]
pub enum SelectData {
    String(String),
    Number(f64),
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = "index.ts")]
pub enum SelectDataType {
    String,
    Number,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "index.ts")]
pub enum CodeLanguage {
    Js,
    Css,
    Html,
    Json,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct RangeProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    min: f64,
    max: f64,
    step: Option<f64>,
    #[ts(type = "Record<string, string>")]
    max_label: HashMap<String, String>,
    #[ts(type = "Record<string, string>")]
    min_label: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct ButtongroupProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    options: Vec<ButtonGroupOption>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "index.ts")]
pub struct ButtonGroupOption {
    #[ts(type = "Record<string, string>")]
    label: HashMap<String, String>,
    value: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = "index.ts")]
pub enum TailwindColor {
    Red,
    Orange,
    Amber,
    Yellow,
    Lime,
    Green,
    Emerald,
    Teal,
    Cyan,
    Sky,
    Blue,
    Indigo,
    Violet,
    Purple,
    Fuchsia,
    Pink,
    Rose,
    Slate,
    Gray,
    Zinc,
    Neutral,
    Stone,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = "index.ts")]
pub enum Variant {
    Solid,
    Alternative,
}

#[derive(Deserialize, Serialize, FromRow, Clone, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "index.ts")]
pub struct Settings {
    pub cleverreach_client_id: Option<String>,
    pub cleverreach_secret: Option<String>,
}
