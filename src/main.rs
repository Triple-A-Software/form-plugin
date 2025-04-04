use std::{
    env,
    io::BufWriter,
    net::{IpAddr, SocketAddr},
};

use api::{email::email_router, form::form_router};
use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use model::Form;
use render::{Render, Renderer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use tera::Tera;
use tokio::net::TcpListener;
use tower_http::{normalize_path::NormalizePathLayer, services::ServeDir};

mod date;
mod model;
mod render;
mod api {
    pub mod email;
    pub mod form;
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
    tera: Tera,
}

#[tokio::main]
async fn main() {
    let db = create_db().await;

    let tera = Tera::new("./templates/*.html").expect("Failed to parse template files");

    let state = AppState { db, tera };
    let router = Router::new()
        .nest_service("/ui", ServeDir::new("/ui"))
        .merge(form_router())
        .merge(email_router())
        .route("/helper/render_form", post(render_form_helper))
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

#[derive(Deserialize)]
struct HelperBody {
    args: Vec<Value>,
}
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub enum ServicePluginApiResponse<T>
where
    T: std::fmt::Debug,
{
    Error(String),
    Data(T),
}

async fn render_form_helper(
    State(state): State<AppState>,
    Json(body): Json<HelperBody>,
) -> impl IntoResponse {
    if let Some(Value::Number(id)) = body.args.first() {
        if let Some(id) = id.as_i64() {
            let renderer = Renderer::new("en", state.tera);
            let form: Form = sqlx::query_as(r#"select * from form where id = $1"#)
                .bind(id as i32)
                .fetch_one(&state.db)
                .await
                .unwrap();
            let mut output = BufWriter::new(Vec::new());
            for prop in form.props.iter() {
                prop.render(&renderer, &mut output);
            }
            let rendered = String::from_utf8(output.into_inner().unwrap()).unwrap();
            Json(ServicePluginApiResponse::Data(Value::String(rendered)))
        } else {
            Json(ServicePluginApiResponse::Error(
                "invalid i32 argument".to_string(),
            ))
        }
    } else {
        Json(ServicePluginApiResponse::Data(Value::String(
            "".to_string(),
        )))
    }
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
