use std::env;

use cleverreach::Cleverreach;
use model::{Email, Form};
use serde_json::Value;
use sqlx::PgPool;
use tera::Tera;

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
    pub cleverreach: Cleverreach,
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
