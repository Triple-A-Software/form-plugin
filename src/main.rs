use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use api::{email::email_router, form::form_router, submission::submission_router};
use axum::{
    Json, Router,
    extract::{self, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::post,
};
use model::Form;
use render::render_form;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use tera::Tera;
use tokio::net::TcpListener;
use tower_http::{
    normalize_path::NormalizePathLayer,
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::Level;
use url::Url;

mod date;
mod model;
mod render;
mod api {
    pub mod email;
    pub mod form;
    pub mod submission;
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
    tera: Tera,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let db = create_db().await;

    let tera = Tera::new("./templates/*.html").expect("Failed to parse template files");

    let state = AppState { db, tera };
    let router = Router::new()
        .nest_service("/ui", ServeDir::new("ui/dist"))
        .merge(form_router())
        .merge(email_router())
        .merge(submission_router())
        .route("/helper/render_form", post(render_form_helper))
        .route("/form-plugin/submit", post(form_submit))
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
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
            let form: Form = sqlx::query_as(r#"select * from form where id = $1"#)
                .bind(id)
                .fetch_one(&state.db)
                .await
                .unwrap();
            let mut form_action_url = Url::parse("/form-plugin/submit").unwrap();
            let route = ""; //TODO this will be passed in from the handlebars service
            {
                let mut action_query = form_action_url.query_pairs_mut();
                action_query.append_pair("id", &id.to_string());
                action_query.append_pair("redirect", route);
            }
            let rendered = render_form(
                state,
                "en",
                form,
                format!(r#"<form action="{form_action_url}" method="POST">"#,),
                "</form>",
            );
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

#[derive(Deserialize, Debug, Clone)]
struct FormSubmitQuery {
    id: i32,
    redirect: Option<String>,
}
async fn form_submit(
    State(state): State<AppState>,
    Query(query): Query<FormSubmitQuery>,
    extract::Form(body): extract::Form<Value>,
) -> impl IntoResponse {
    sqlx::query(r#"insert into submission (form_id, data) value ($1, $2)"#)
        .bind(query.id)
        .bind(sqlx::types::Json(body))
        .execute(&state.db)
        .await
        .unwrap();
    if let Some(redirect) = query.redirect {
        Redirect::temporary(&redirect).into_response()
    } else {
        StatusCode::OK.into_response()
    }
}
