use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::*, types::Json};
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
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
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
    #[ts(as = "Vec<Property>")]
    pub props: Json<Vec<Property>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS, FromRow)]
#[ts(export, export_to = "index.ts")]
pub struct Email {
    pub id: i32,
    pub subject: String,
    /// html code for the email body
    pub content: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[ts(export, export_to = "index.ts")]
pub enum Property {
    Textarea(TextareaProperty),
    Text(TextProperty),
    Markdown(PropertyCommon),
    Richtext(PropertyCommon),
    Checkbox(PropertyCommon),
    Number(NumberProperty),
    Select(SelectProperty),
    Code(CodeProperty),
    Range(RangeProperty),
    Buttongroup(ButtongroupProperty),
    Button(PropertyCommon),
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct TextareaProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    max_length: Option<i32>,
    placeholder: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct TextProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    max: Option<i32>,
    placeholder: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct NumberProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
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
#[ts(export, export_to = "index.ts")]
pub struct CodeProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    language: CodeLanguage,
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
