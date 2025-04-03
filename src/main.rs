use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use api::{email::email_router, form::form_router};
use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::{normalize_path::NormalizePathLayer, services::ServeDir};

mod date;
mod model;
mod api {
    pub mod email;
    pub mod form;
}

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
        .merge(form_router())
        .merge(email_router())
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

async fn create_db() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = sqlx::PgPool::connect(&db_url).await.unwrap();
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");
    db
}
