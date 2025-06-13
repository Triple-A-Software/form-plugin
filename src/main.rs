use std::{
    collections::HashMap,
    env,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use axum::{
    Json, Router,
    extract::{self, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use form_plugin::render::render_form;
use form_plugin::{
    AppState,
    api::{
        email::email_router, form::form_router, settings::settings_router,
        submission::submission_router,
    },
};
use form_plugin::{captcha::Captcha, extract_field_as_string, send_email};
use form_plugin::{
    cleverreach::{Cleverreach, ReceiverRequest},
    create_db,
};
use form_plugin::{
    database::create_submission,
    model::{CleverreachFormCondition, Form, Settings, Submission},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tera::Tera;
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::{
    normalize_path::NormalizePathLayer,
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let db = create_db().await;

    // this query needs the do update part, because otherwise we would need another query to fetch
    // the settings, because insert on conflict do nothing only returns something when inserting,
    // not when doing nothing
    let settings: Settings = sqlx::query_as(
        r#"insert into settings (id) values ('settings') on conflict (id) do update set id = 'settings' returning *"#,
    )
    .fetch_one(&db)
    .await
    .unwrap();

    let tera = Tera::new("./templates/*.html").expect("Failed to parse template files");

    let state = AppState {
        db,
        tera,
        cleverreach: Arc::new(RwLock::new(Cleverreach::new(
            settings.cleverreach_client_id,
            settings.cleverreach_secret,
        ))),
    };
    let router = Router::new()
        .nest_service("/ui", ServeDir::new("ui/dist"))
        .merge(form_router())
        .merge(email_router())
        .merge(submission_router())
        .merge(settings_router())
        .route("/helper/render_form", post(render_form_helper))
        .route("/form-plugin/submit", post(form_submit))
        .route("/test", get(test_page))
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
struct TestPageQuery {
    id: i32,
}
async fn test_page(
    State(state): State<AppState>,
    Query(TestPageQuery { id }): Query<TestPageQuery>,
) -> impl IntoResponse {
    let form: Form = sqlx::query_as(r#"select * from form where id = $1"#)
        .bind(id)
        .fetch_one(&state.db)
        .await
        .unwrap();
    let settings: Settings = sqlx::query_as(r#"select * from settings where id = 'settings'"#)
        .fetch_one(&state.db)
        .await
        .unwrap();
    let rendered = render_form(
        state,
        "en".to_string(),
        form,
        format!(r#"<form action="/form-plugin/submit?id={id}&redirect=/test" method="POST">"#,),
        "</form>",
        settings,
    );
    Html(rendered)
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct HelperBody {
    json_args: Vec<Value>,
    page: PageRenderInput,
    query: HashMap<String, String>,
    params: HashMap<String, String>,
    route: String,
    interactive: bool,
}
#[derive(Deserialize)]
#[allow(dead_code)]
/// this can get many more fields, but we don't necessarily need those
struct PageRenderInput {
    language: Option<String>,
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
    if let Some(([Value::Number(id), Value::String(route)], _)) =
        body.json_args.split_first_chunk::<2>()
    {
        if let Some(id) = id.as_i64() {
            // we need to cast the id to an int with `id::int`, because otherwise we randomly get an error,
            // that there is not enough data left in message or the binary data in the parameter 1 is
            // incorrect, even though the id column is defined as an `int` which according to the
            // documentation of sqlx, is mapped to an i32
            let form: Form = match sqlx::query_as(r#"select * from form where id::int = $1"#)
                .bind(id as i32)
                .fetch_one(&state.db)
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    if body.interactive {
                        return Json(ServicePluginApiResponse::Data(Value::String(format!(
                            "Error when loading form from database: {e}"
                        ))));
                    }
                    return Json(ServicePluginApiResponse::Data(Value::String(
                        "".to_string(),
                    )));
                }
            };
            let settings: Settings =
                sqlx::query_as(r#"select * from settings where id = 'settings'"#)
                    .fetch_one(&state.db)
                    .await
                    .unwrap();

            let rendered = render_form(
                state,
                body.page.language.unwrap_or("en".to_string()),
                form,
                format!(
                    r#"<form action="/form-plugin/submit?id={id}&redirect={route}" method="POST">"#,
                ),
                "</form>",
                settings,
            );
            Json(ServicePluginApiResponse::Data(Value::String(rendered)))
        } else {
            Json(ServicePluginApiResponse::Error(
                "invalid i32 argument".to_string(),
            ))
        }
    } else if body.interactive {
        Json(ServicePluginApiResponse::Data(Value::String(
            "Invalid number of arguments, wanted: render_form(int, string)".to_string(),
        )))
    } else {
        Json(ServicePluginApiResponse::Data(Value::String(
            "".to_string(),
        )))
    }
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
    let form: Option<Form> = sqlx::query_as(r#"select * from form where id = $1"#)
        .bind(query.id)
        .fetch_optional(&state.db)
        .await
        .unwrap();
    if let Some(form) = form {
        let settings: Settings = sqlx::query_as(r#"select * from settings where id = 'settings'"#)
            .fetch_one(&state.db)
            .await
            .unwrap();
        if form.activate_captcha {
            let captcha_response = match form.captcha_type {
                Some(ref r#type) => {
                    Captcha {
                        body: &body,
                        r#type,
                    }
                    .handle(settings)
                    .await
                }
                None => None,
            };
            if let Some(captcha_response) = captcha_response {
                if !captcha_response.success {
                    return StatusCode::UNAUTHORIZED.into_response();
                }
            } else {
                return StatusCode::UNAUTHORIZED.into_response();
            }
        }
        if form.activate_cleverreach {
            let should_create_receiver = match form.cleverreach_condition.0 {
                CleverreachFormCondition::OnSubmit => {
                    // when it is on submit, we don't need to check anything additionally
                    true
                }
                CleverreachFormCondition::WhenFieldIsSet { ref field } => {
                    is_field_set(&body, Some(field))
                }
            };
            if should_create_receiver {
                let first_name = extract_field_as_string(
                    &body,
                    form.cleverreach_firstname_field_name.as_deref(),
                );
                let last_name =
                    extract_field_as_string(&body, form.cleverreach_lastname_field_name.as_deref());
                let email =
                    extract_field_as_string(&body, form.cleverreach_email_field_name.as_deref());
                if let Some(email) = email {
                    if let Some(group_id) = &form.cleverreach_group_id {
                        // the form is used to sign up for a cleverreach newsletter
                        state
                            .cleverreach
                            .write()
                            .await
                            .create_receiver(ReceiverRequest {
                                group_id: group_id.to_string(),
                                email,
                                first_name,
                                last_name,
                                salutation: form.cleverreach_salutation.clone().unwrap_or_default(),
                            })
                            .await
                            .unwrap();
                    }
                }
            }
        }
        if form.send_submission_as_email_enabled {
            if let (Some(submission_receiver_email), Some(submissions_email_id)) =
                (&form.send_submission_as_email, &form.submissions_email_id)
            {
                send_email(
                    &state.db,
                    *submissions_email_id,
                    &form,
                    &body,
                    submission_receiver_email,
                )
                .await;
            }
        }
        if form.send_confirmation_email {
            if let (Some(confirmation_email_recipient), Some(confirmation_email_id)) = (
                extract_field_as_string(
                    &body,
                    form.confirmation_email_recipient_field_name.as_deref(),
                ),
                &form.confirmation_email_id,
            ) {
                send_email(
                    &state.db,
                    *confirmation_email_id,
                    &form,
                    &body,
                    &confirmation_email_recipient,
                )
                .await;
            }
        }
        create_submission(
            Submission {
                form_id: query.id,
                data: sqlx::types::Json(body),
                ..Default::default()
            },
            &state.db,
        )
        .await
        .unwrap();
        if let Some(redirect) = query.redirect {
            Redirect::to(&redirect).into_response()
        } else {
            StatusCode::OK.into_response()
        }
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

fn is_field_set(data: &Value, field_name: Option<&str>) -> bool {
    if let Some(field_name) = field_name {
        if let Some(field) = data.get(field_name) {
            match field {
                Value::Null => false,
                Value::Bool(v) => *v,
                Value::Number(number) => {
                    if let Some(number) = number.as_i64() {
                        number != 0
                    } else if let Some(number) = number.as_f64() {
                        number != 0.0
                    } else if let Some(number) = number.as_u64() {
                        number != 0
                    } else {
                        false
                    }
                }
                Value::String(s) => !s.is_empty(),
                Value::Array(vec) => !vec.is_empty(),
                Value::Object(map) => !map.is_empty(),
            }
        } else {
            false
        }
    } else {
        false
    }
}
