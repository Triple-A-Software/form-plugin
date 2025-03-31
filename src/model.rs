use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct PropertyCommon {
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[ts(type = "Record<string, string>")]
    label: HashMap<String, String>,
    #[ts(optional, type = "unknown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<Value>,
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    pub id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct Builder {
    data: Value,
    name: String,
    icon: Option<String>,
    props: Vec<Property>,
    #[serde(flatten)]
    #[ts(skip)]
    extra: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[ts(export, export_to = "index.ts")]
pub enum Property {
    Textarea(PropertyCommon),
    Password(PropertyCommon),
    Text(PropertyCommon),
    Markdown(PropertyCommon),
    Richtext(PropertyCommon),
    Checkbox(PropertyCommon),
    Number(NumberProperty),
    Select(SelectProperty),
    Code(CodeProperty),
    Range(RangeProperty),
    Buttongroup(ButtongroupProperty),
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct NumberProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<f64>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
pub struct SelectProperty {
    #[serde(flatten)]
    common: PropertyCommon,
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    multiple: Option<bool>,
    data_type: SelectDataType,
    options: SelectOptions,
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
#[serde(rename_all = "camelCase", untagged)]
#[ts(export, export_to = "index.ts")]
pub enum SelectOptions {
    Remote(String),
    Local(Vec<SelectOption>),
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
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
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
