use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use axum::Router;
use tokio::net::TcpListener;
use tower_http::{normalize_path::NormalizePathLayer, services::ServeDir};

#[derive(Clone)]
struct AppState {}

#[tokio::main]
async fn main() {
    let state = AppState {};
    let router = Router::new()
        .nest_service("/ui", ServeDir::new("/ui"))
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
