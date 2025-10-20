use std::collections::{HashMap, HashSet};
use std::env;
use std::io::Cursor;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use cleverreach::Cleverreach;
use model::{Email, Form};
use serde_json::Value;
use sqlx::{PgConnection, PgPool};
use tera::Tera;
use tokio::sync::RwLock;

use crate::model::{EmailAttachment, Submission};

pub mod captcha;
pub mod cleverreach;
pub mod database;
pub mod date;
pub mod model;
pub mod render;
pub mod utils;
pub mod api {
    pub mod dashboard;
    pub mod email;
    pub mod form;
    pub mod settings;
    pub mod submission;
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub tera: Tera,
    pub cleverreach: Arc<RwLock<Cleverreach>>,
}

pub async fn create_db() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = sqlx::PgPool::connect(&db_url).await.unwrap();
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");
    db
}

pub async fn send_email(
    db: &mut PgConnection,
    id: i32,
    form: &Form,
    body: &Value,
    to: &str,
    attachments: Option<Vec<EmailAttachment>>,
) {
    let email: Email = sqlx::query_as(r#"select * from email where id = $1"#)
        .bind(id)
        .fetch_one(db)
        .await
        .unwrap();
    let mut context = tera::Context::new();
    context.insert("form", &form);
    context.insert("submission", &body);
    email.send(to, context, attachments).await.unwrap();
}

pub fn extract_field_as_string(data: &Value, field_name: Option<&str>) -> Option<String> {
    if let Some(field_name) = field_name {
        Some(data.get(field_name).map(|v| {
            if let Value::String(v) = v {
                v.to_string()
            } else {
                v.to_string()
            }
        }))
        .flatten()
    } else {
        None
    }
}

pub fn convert_to_csv(submissions: &[Submission]) -> String {
    let mut all_keys = HashSet::from(["id", "form_id", "timestamp"]);
    let mut output = Vec::new();
    {
        let cursor = Cursor::new(&mut output);
        let mut writer = csv::Writer::from_writer(cursor);

        let submissions: Vec<HashMap<String, Value>> = submissions
            .iter()
            .map(|submission| {
                let mut map = HashMap::new();
                map.insert("id".to_string(), Value::String(submission.id.to_string()));
                map.insert("form_id".to_string(), Value::from(submission.form_id));
                map.insert(
                    "timestamp".to_string(),
                    Value::from(DateTime::<Utc>::from(submission.timestamp.clone()).to_rfc3339()),
                );
                if let Value::Object(ref data) = *submission.data {
                    for (key, value) in data.iter() {
                        all_keys.insert(key);
                        map.insert(key.to_string(), value.clone());
                    }
                }
                map
            })
            .collect();

        writer.write_record(&all_keys).unwrap();
        for row in submissions {
            let record = all_keys
                .iter()
                .map(|k| {
                    row.get(*k)
                        .map(|v| match v {
                            Value::String(v) => v.to_string(),
                            Value::Number(v) => v.to_string(),
                            Value::Bool(v) => v.to_string(),
                            Value::Null => "".to_string(),
                            v => v.to_string(),
                        })
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>();
            writer.write_record(&record).unwrap();
        }
    }
    String::from_utf8(output).unwrap()
}
