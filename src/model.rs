use std::{collections::HashMap, env, fmt::Display, path::PathBuf};

#[cfg(feature = "seed")]
use fake::{
    Dummy, Fake, Faker, Rng,
    faker::{
        filesystem::en::DirPath,
        lorem::{
            de_de,
            en::{self, Word},
        },
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::*, types::Json};
use tera::Context;
use ts_rs::TS;
use uuid::Uuid;

#[cfg(feature = "seed")]
struct CustomFaker;

#[cfg(feature = "seed")]
impl<O: Dummy<Faker>> Dummy<CustomFaker> for Json<O> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CustomFaker, rng: &mut R) -> Self {
        Json(Faker.fake_with_rng(rng))
    }
}

#[cfg(feature = "seed")]
struct TranslationFaker;
#[cfg(feature = "seed")]
impl Dummy<TranslationFaker> for HashMap<String, String> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &TranslationFaker, rng: &mut R) -> Self {
        let mut map = HashMap::new();
        map.insert("en".to_string(), en::Word().fake_with_rng(rng));
        map.insert("de".to_string(), de_de::Word().fake_with_rng(rng));
        map
    }
}

use crate::{captcha::CaptchaType, date::JavascriptDate};

#[derive(Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct StatusResponse {
    success: bool,
}

impl StatusResponse {
    pub fn success() -> Self {
        Self { success: true }
    }

    pub fn fail() -> Self {
        Self { success: false }
    }
}

#[derive(Serialize, TS, Debug, Clone)]
#[ts(export, export_to = "index.ts")]
pub struct ApiError {
    error_message: String,
}

#[derive(Deserialize, Serialize, Default, Clone, Debug, TS, FromRow)]
#[cfg_attr(feature = "seed", derive(Dummy))]
#[ts(export, export_to = "index.ts")]
pub struct Submission {
    pub id: Uuid,
    #[cfg_attr(feature = "seed", dummy(expr = "1"))]
    pub form_id: i32,
    pub timestamp: JavascriptDate,
    #[ts(as = "Value")]
    #[cfg_attr(feature = "seed", dummy(faker = "CustomFaker"))]
    pub data: Json<Value>,
}

#[derive(Serialize, TS, FromRow)]
#[ts(export, export_to = "index.ts")]
pub struct SubmissionApiResponse {
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub submission: Submission,
    pub file_count: i64,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS, FromRow)]
#[ts(export, export_to = "index.ts")]
pub struct Form {
    #[cfg_attr(feature = "seed", dummy(expr = "1"))]
    pub id: i32,
    #[cfg_attr(feature = "seed", dummy(faker = "Word()"))]
    pub name: String,
    pub send_confirmation_email: bool,
    #[cfg_attr(feature = "seed", dummy(expr = "Some(1)"))]
    pub confirmation_email_id: Option<i32>,
    #[cfg_attr(feature = "seed", dummy(faker = "Word()"))]
    pub confirmation_email_recipient_field_name: Option<String>,
    pub send_submission_as_email_enabled: bool,
    pub send_submission_as_email: Option<String>,
    #[cfg_attr(feature = "seed", dummy(expr = "Some(1)"))]
    pub submissions_email_id: Option<i32>,
    pub activate_cleverreach: bool,
    #[ts(as = "CleverreachFormCondition")]
    #[cfg_attr(feature = "seed", dummy(faker = "CustomFaker"))]
    pub cleverreach_condition: Json<CleverreachFormCondition>,
    #[cfg_attr(feature = "seed", dummy(faker = "0.."))]
    pub cleverreach_group_id: Option<String>,
    #[cfg_attr(feature = "seed", dummy(faker = "Word()"))]
    pub cleverreach_email_field_name: Option<String>,
    #[cfg_attr(feature = "seed", dummy(faker = "Word()"))]
    pub cleverreach_firstname_field_name: Option<String>,
    #[cfg_attr(feature = "seed", dummy(faker = "Word()"))]
    pub cleverreach_lastname_field_name: Option<String>,
    #[cfg_attr(feature = "seed", dummy(faker = "Word()"))]
    pub cleverreach_salutation: Option<String>,
    pub activate_captcha: bool,
    pub captcha_type: Option<CaptchaType>,
    #[cfg_attr(feature = "seed", dummy(faker = "DirPath()"))]
    pub captcha_callback: Option<String>,
    #[cfg_attr(feature = "seed", dummy(faker = "DirPath()"))]
    pub captcha_expired_callback: Option<String>,
    #[ts(as = "Vec<Property>")]
    #[cfg_attr(feature = "seed", dummy(faker = "CustomFaker"))]
    pub props: Json<Vec<Property>>,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(tag = "type")]
#[ts(export, export_to = "index.ts")]
pub enum CleverreachFormCondition {
    OnSubmit,
    WhenFieldIsSet {
        #[cfg_attr(feature = "seed", dummy(faker = "Word()"))]
        field: String,
    },
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS, FromRow)]
#[ts(export, export_to = "index.ts")]
pub struct Email {
    #[cfg_attr(feature = "seed", dummy(expr = "1"))]
    pub id: i32,
    #[cfg_attr(feature = "seed", dummy(faker = "Word()"))]
    pub subject: String,
    /// html code for the email body
    pub content: String,
}

impl Email {
    pub async fn send(
        &self,
        receiver: &str,
        render_context: Context,
        attachments: Option<Vec<EmailAttachment>>,
    ) -> anyhow::Result<()> {
        let internal_backend_port = env::var("INTERNAL_BACKEND_PORT")?;
        reqwest::Client::new()
            .post(format!(
                "http://localhost:{internal_backend_port}/email/send"
            ))
            .json(&EmailSendRequest {
                receiver,
                subject: self.subject.clone(),
                content: tera::Tera::one_off(&self.content, &render_context, true).unwrap(),
                attachments,
            })
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct EmailAttachment {
    pub name: String,
    /// this needs to be the full path to the file
    pub path: PathBuf,
    pub mime_type: String,
}

#[derive(Serialize)]
pub struct EmailSendRequest<'a> {
    pub receiver: &'a str,
    pub subject: String,
    pub content: String,
    pub attachments: Option<Vec<EmailAttachment>>,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
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
    File(FileProperty),
    Date(DateProperty),
    Captcha(CaptchaProperty),
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct PropertyCommon {
    #[ts(type = "Record<string, string>")]
    #[cfg_attr(feature = "seed", dummy(faker = "TranslationFaker"))]
    hint: HashMap<String, String>,
    #[ts(type = "Record<string, string>")]
    #[cfg_attr(feature = "seed", dummy(faker = "TranslationFaker"))]
    label: HashMap<String, String>,
    default_value: Option<Value>,
    #[cfg_attr(feature = "seed", dummy(faker = "Word()"))]
    class: String,
    #[cfg_attr(feature = "seed", dummy(faker = "Word()"))]
    pub name: String,
    required: bool,
    color: TailwindColor,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct FileProperty {
    #[serde(flatten)]
    pub common: PropertyCommon,
    label_class: Option<String>,
    input_class: Option<String>,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct DateProperty {
    #[serde(flatten)]
    pub common: PropertyCommon,
    #[ts(type = "Record<string, string>")]
    #[cfg_attr(feature = "seed", dummy(faker = "TranslationFaker"))]
    placeholder: HashMap<String, String>,
    pub input_type: DateInputType,
    label_class: Option<String>,
    input_class: Option<String>,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = "index.ts")]
pub enum DateInputType {
    Date,
    DatetimeLocal,
    Time,
    Month,
    Week,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct ButtonProperty {
    #[serde(flatten)]
    pub common: PropertyCommon,
    button_type: ButtonType,
    variant: Variant,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
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

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct CheckboxProperty {
    #[serde(flatten)]
    pub common: PropertyCommon,
    label_class: Option<String>,
    input_class: Option<String>,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct TextareaProperty {
    #[serde(flatten)]
    pub common: PropertyCommon,
    #[cfg_attr(feature = "seed", dummy(faker = "0.."))]
    max_length: Option<i32>,
    #[ts(type = "Record<string, string>")]
    #[cfg_attr(feature = "seed", dummy(faker = "TranslationFaker"))]
    placeholder: HashMap<String, String>,
    #[cfg_attr(feature = "seed", dummy(faker = "0.."))]
    rows: Option<i32>,
    #[cfg_attr(feature = "seed", dummy(faker = "0.."))]
    columns: Option<i32>,
    input_class: Option<String>,
    label_class: Option<String>,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct TextProperty {
    #[serde(flatten)]
    pub common: PropertyCommon,
    #[cfg_attr(feature = "seed", dummy(faker = "0.."))]
    max: Option<i32>,
    #[ts(type = "Record<string, string>")]
    #[cfg_attr(feature = "seed", dummy(faker = "TranslationFaker"))]
    placeholder: HashMap<String, String>,
    input_type: TextInputType,
    input_class: Option<String>,
    label_class: Option<String>,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
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

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct NumberProperty {
    #[serde(flatten)]
    pub common: PropertyCommon,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    #[ts(type = "Record<string, string>")]
    #[cfg_attr(feature = "seed", dummy(faker = "TranslationFaker"))]
    placeholder: HashMap<String, String>,
    label_class: Option<String>,
    input_class: Option<String>,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct SelectProperty {
    #[serde(flatten)]
    pub common: PropertyCommon,
    multiple: Option<bool>,
    pub data_type: SelectDataType,
    options: Vec<SelectOption>,
    label_class: Option<String>,
    input_class: Option<String>,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "index.ts")]
pub struct SelectOption {
    #[ts(type = "Record<string, string>")]
    #[cfg_attr(feature = "seed", dummy(faker = "TranslationFaker"))]
    label: HashMap<String, String>,
    value: SelectData,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "kebab-case", untagged)]
#[ts(export, export_to = "index.ts")]
pub enum SelectData {
    String(#[cfg_attr(feature = "seed", dummy(faker = "Word()"))] String),
    Number(f64),
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = "index.ts")]
pub enum SelectDataType {
    String,
    Number,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct RangeProperty {
    #[serde(flatten)]
    pub common: PropertyCommon,
    pub min: f64,
    pub max: f64,
    pub step: Option<f64>,
    #[ts(type = "Record<string, string>")]
    #[cfg_attr(feature = "seed", dummy(faker = "TranslationFaker"))]
    max_label: HashMap<String, String>,
    #[ts(type = "Record<string, string>")]
    #[cfg_attr(feature = "seed", dummy(faker = "TranslationFaker"))]
    min_label: HashMap<String, String>,
    label_class: Option<String>,
    input_class: Option<String>,
    min_label_class: Option<String>,
    max_label_class: Option<String>,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct ButtongroupProperty {
    #[serde(flatten)]
    pub common: PropertyCommon,
    pub options: Vec<ButtonGroupOption>,
    button_class: Option<String>,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "index.ts")]
pub struct ButtonGroupOption {
    #[ts(type = "Record<string, string>")]
    #[cfg_attr(feature = "seed", dummy(faker = "TranslationFaker"))]
    label: HashMap<String, String>,
    pub value: String,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "index.ts")]
pub struct CaptchaProperty {
    #[cfg_attr(feature = "seed", dummy(faker = "Word()"))]
    /// this is automatically filled based on the captcha_type
    pub captcha_class: String,
    #[cfg_attr(feature = "seed", dummy(faker = "Word()"))]
    /// this is automatically filled from the settings
    pub sitekey: String,
    #[cfg_attr(feature = "seed", dummy(faker = "DirPath()"))]
    pub callback: Option<String>,
    #[cfg_attr(feature = "seed", dummy(faker = "DirPath()"))]
    pub expired_callback: Option<String>,
    pub captcha_type: CaptchaType,
}

#[cfg_attr(feature = "seed", derive(Dummy))]
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

#[cfg_attr(feature = "seed", derive(Dummy))]
#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = "index.ts")]
pub enum Variant {
    Solid,
    Alternative,
    Unstyled,
}

#[derive(Deserialize, Serialize, FromRow, Clone, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "index.ts")]
pub struct Settings {
    pub cleverreach_client_id: Option<String>,
    pub cleverreach_secret: Option<String>,
    pub h_captcha_secret: Option<String>,
    pub re_captcha_secret: Option<String>,
    pub captcha_sitekey: Option<String>,
}

impl Settings {
    pub fn get_captcha_secret(&self, captcha_type: &CaptchaType) -> Option<&str> {
        match captcha_type {
            CaptchaType::HCaptcha => self.h_captcha_secret.as_deref(),
            CaptchaType::ReCaptcha => self.re_captcha_secret.as_deref(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SubmissionFile {
    /// this also serves as the file name in the FILE_STORAGE_PATH directory
    pub id: Uuid,
    pub name: Option<String>,
    pub submission_id: Uuid,
    pub mime_type: String,
}
