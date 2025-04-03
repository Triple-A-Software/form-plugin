use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use axum::{Json, Router, extract::State, routing::get};
use model::{Email, Form, StatusResponse};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::{normalize_path::NormalizePathLayer, services::ServeDir};

mod date;
mod model;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    let db = create_db().await;

    let state = AppState { db };
    let router = Router::new()
        .nest_service("/ui", ServeDir::new("/ui"))
        .route(
            "/api/forms",
            get(list_forms).post(create_form).put(update_form),
        )
        .route("/api/emails", get(list_emails))
        .layer(NormalizePathLayer::trim_trailing_slash())
        .with_state(state);

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let listener = TcpListener::bind((
        "0.0.0.0".parse::<IpAddr>().unwrap(),
        port.parse::<u16>().unwrap(),
    ))
    .await
    .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn create_form(
    State(state): State<AppState>,
    Json(body): Json<Form>,
) -> Json<StatusResponse> {
    sqlx::query(
        r#"insert into form (
            name,
            send_confirmation_email,
            confirmation_email_id,
            confirmation_email_recipient_field_name,
            props
        ) values ($1, $2, $3, $4, $5)"#,
    )
    .bind(body.name)
    .bind(body.send_confirmation_email)
    .bind(body.confirmation_email_id)
    .bind(body.confirmation_email_recipient_field_name)
    .bind(body.props)
    .execute(&state.db)
    .await
    .unwrap();
    Json(StatusResponse::success())
}

async fn update_form(
    State(state): State<AppState>,
    Json(body): Json<Form>,
) -> Json<StatusResponse> {
    sqlx::query(
        r#"update form set
            name = $1,
            send_confirmation_email = $2,
            confirmation_email_id = $3,
            confirmation_email_recipient_field_name = $4,
            props = $5 where id = $6"#,
    )
    .bind(body.name)
    .bind(body.send_confirmation_email)
    .bind(body.confirmation_email_id)
    .bind(body.confirmation_email_recipient_field_name)
    .bind(body.props)
    .bind(body.id)
    .execute(&state.db)
    .await
    .unwrap();
    Json(StatusResponse::success())
}

async fn list_forms(State(state): State<AppState>) -> Json<Vec<Form>> {
    Json(
        sqlx::query_as(r#"select * from form"#)
            .fetch_all(&state.db)
            .await
            .unwrap(),
    )
}

async fn list_emails(State(state): State<AppState>) -> Json<Vec<Email>> {
    Json(
        sqlx::query_as(r#"select * from email"#)
            .fetch_all(&state.db)
            .await
            .unwrap(),
    )
}

async fn create_db() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = sqlx::PgPool::connect(&db_url).await.unwrap();
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");
    db
}
