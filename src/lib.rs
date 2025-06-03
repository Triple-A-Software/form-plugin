use std::env;
use std::{collections::HashSet, sync::Arc};

use cleverreach::Cleverreach;
use model::{Email, Form};
use serde_json::Value;
use sqlx::PgPool;
use tera::Tera;
use tokio::sync::RwLock;

pub mod captcha;
pub mod cleverreach;
pub mod database;
pub mod date;
pub mod model;
pub mod render;
pub mod api {
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

pub async fn send_email(db: &PgPool, id: i32, form: &Form, body: &Value, to: &str) {
    let email: Email = sqlx::query_as(r#"select * from email where id = $1"#)
        .bind(id)
        .fetch_one(db)
        .await
        .unwrap();
    let mut context = tera::Context::new();
    context.insert("form", &form);
    context.insert("submission", &body);
    email.send(to, context).await.unwrap();
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

pub fn convert_to_csv(arr: &[serde_json::Value]) -> String {
    // Collect all unique keys from all objects
    let mut all_keys = HashSet::new();

    fn extract_keys(obj: &serde_json::Value, prefix: &str, all_keys: &mut HashSet<String>) {
        if let Some(obj_map) = obj.as_object() {
            for (key, value) in obj_map {
                let mut _key = key.clone();
                if !prefix.is_empty() {
                    _key = format!("{}.{}", prefix, key);
                }
                if value.is_object() {
                    extract_keys(value, &_key, all_keys);
                } else {
                    all_keys.insert(_key);
                }
            }
        }
    }

    for obj in arr {
        extract_keys(obj, "", &mut all_keys);
    }

    let headers: Vec<String> = all_keys.into_iter().collect();

    // Create the header row
    let mut csv_rows = vec![headers.join(",")];

    // Create the data rows
    for row in arr {
        let values: Vec<String> = headers
            .iter()
            .map(|header| {
                let header_path: Vec<&str> = header.split('.').collect();
                let mut value = row.clone();
                for key in header_path {
                    if let Some(v) = value.get(key) {
                        value = v.clone();
                    } else {
                        value = serde_json::Value::String(String::new());
                        break;
                    }
                }

                let escaped_value = match value.as_str() {
                    Some(v) => v.replace("\"", "\"\""),
                    None => value.to_string(),
                };

                if escaped_value.contains(',') || escaped_value.contains('"') {
                    format!("\"{}\"", escaped_value)
                } else {
                    escaped_value
                }
            })
            .collect();

        csv_rows.push(values.join(","));
    }

    // Join all rows with newline characters
    csv_rows.join("\n")
}
