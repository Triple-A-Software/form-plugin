use std::{
    collections::HashMap,
    env,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use axum::{
    Json, Router,
    extract::{Multipart, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use chrono::NaiveDate;
use chrono::{Datelike, NaiveTime};
use form_plugin::{
    AppState,
    api::{
        email::email_router, form::form_router, settings::settings_router,
        submission::submission_router,
    },
    captcha::CaptchaType,
    model::{DateInputType, EmailAttachment, Property, SelectDataType, SubmissionFile},
    utils::{get_file_path, get_file_storage_path, stream_to_file},
};
use form_plugin::{api::dashboard::dashboard_router, render::render_form};
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
use uuid::Uuid;

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
        .merge(dashboard_router())
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
        format!(
            r#"<form action="/form-plugin/submit?id={id}&redirect=/test" method="POST" enctype="multipart/form-data">"#,
        ),
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
                    r#"<form action="/form-plugin/submit?id={id}&redirect={route}" method="POST" enctype="multipart/form-data">"#,
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
    mut multipart: Multipart,
) -> impl IntoResponse {
    let form: Option<Form> = sqlx::query_as(r#"select * from form where id = $1"#)
        .bind(query.id)
        .fetch_optional(&state.db)
        .await
        .unwrap();
    if let Some(form) = form {
        let mut tx = state.db.begin().await.unwrap();
        let submission_id = Uuid::new_v4();
        let (body, files) = extract_form_body(&mut multipart, &form, &submission_id).await;
        let settings: Settings = sqlx::query_as(r#"select * from settings where id = 'settings'"#)
            .fetch_one(&mut *tx)
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
                dbg!(&captcha_response.error_codes);
                if !captcha_response.success {
                    return StatusCode::UNAUTHORIZED.into_response();
                } else {
                    tracing::error!("{:?}", &captcha_response.error_codes);
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
                if let Some(email) = email
                    && let Some(group_id) = &form.cleverreach_group_id
                {
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

        if form.send_submission_as_email_enabled
            && let (Some(submission_receiver_email), Some(submissions_email_id)) =
                (&form.send_submission_as_email, &form.submissions_email_id)
        {
            let attachments = files
                .iter()
                .map(|f| EmailAttachment {
                    name: f.name.clone().unwrap_or_default(),
                    path: get_file_path(&f.id),
                    mime_type: f.mime_type.clone(),
                })
                .collect();
            send_email(
                &mut tx,
                *submissions_email_id,
                &form,
                &body,
                submission_receiver_email,
                Some(attachments),
            )
            .await;
        }

        if form.send_confirmation_email
            && let (Some(confirmation_email_recipient), Some(confirmation_email_id)) = (
                extract_field_as_string(
                    &body,
                    form.confirmation_email_recipient_field_name.as_deref(),
                ),
                &form.confirmation_email_id,
            )
        {
            send_email(
                &mut tx,
                *confirmation_email_id,
                &form,
                &body,
                &confirmation_email_recipient,
                None,
            )
            .await;
        }

        create_submission(
            Submission {
                id: submission_id,
                form_id: query.id,
                data: sqlx::types::Json(body),
                ..Default::default()
            },
            &mut *tx,
        )
        .await
        .unwrap();
        for file in files {
            sqlx::query(
                r#"
                    insert into submission_file (id, name, submission_id, mime_type)
                    values ($1, $2, $3, $4) returning *
                "#,
            )
            .bind(file.id)
            .bind(file.name)
            .bind(submission_id)
            .bind(file.mime_type)
            .execute(&mut *tx)
            .await
            .unwrap();
        }
        tx.commit().await.unwrap();

        if let Some(redirect) = query.redirect {
            Redirect::to(&redirect).into_response()
        } else {
            StatusCode::OK.into_response()
        }
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn extract_form_body(
    multipart: &mut Multipart,
    form: &Form,
    submission_id: &Uuid,
) -> (Value, Vec<SubmissionFile>) {
    let mut body = serde_json::Map::new();
    let mut files = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let form_field = form.props.0.iter().find(|&f| match f {
            Property::Textarea(prop) => prop.common.name == name,
            Property::Text(prop) => prop.common.name == name,
            Property::Checkbox(prop) => prop.common.name == name,
            Property::Number(prop) => prop.common.name == name,
            Property::Select(prop) => prop.common.name == name,
            Property::Range(prop) => prop.common.name == name,
            Property::Buttongroup(prop) => prop.common.name == name,
            Property::Button(prop) => prop.common.name == name,
            Property::File(prop) => prop.common.name == name,
            Property::Date(prop) => prop.common.name == name,
            Property::Captcha(_) => match form.captcha_type {
                Some(CaptchaType::HCaptcha) => name == "h-captcha-response",
                Some(CaptchaType::ReCaptcha) => name == "g-recaptcha-response",
                None => false,
            },
        });
        let content = match form_field {
            Some(Property::Textarea(_)) | Some(Property::Text(_)) => {
                Some(Value::String(field.text().await.unwrap()))
            }
            Some(Property::Checkbox(_)) => {
                let text = field.text().await.unwrap();
                Some(Value::Bool(text == "on"))
            }
            Some(Property::Number(prop)) => {
                let text = field.text().await.unwrap();
                parse_number_value(&text, prop.min, prop.max, prop.step)
            }
            Some(Property::Select(prop)) => match prop.data_type {
                SelectDataType::String => Some(Value::String(field.text().await.unwrap())),
                SelectDataType::Number => {
                    parse_number_value(&field.text().await.unwrap(), None, None, None)
                }
            },
            Some(Property::Range(prop)) => {
                let text = field.text().await.unwrap();
                parse_number_value(&text, Some(prop.min), Some(prop.max), prop.step)
            }
            Some(Property::Buttongroup(prop)) => {
                let text = field.text().await.unwrap();
                prop.options
                    .iter()
                    .find(|&o| o.value == text)
                    .map(|value| Value::String(value.value.clone()))
            }
            Some(Property::Button(_)) => None,
            Some(Property::File(_)) => {
                std::fs::create_dir_all(get_file_storage_path()).unwrap();
                let mime_type = field.content_type().unwrap().to_string();
                let file_id = Uuid::new_v4();
                let file_path = get_file_path(&file_id);
                files.push(SubmissionFile {
                    id: file_id,
                    name: field.name().map(|s| s.to_string()),
                    submission_id: *submission_id,
                    mime_type,
                });
                stream_to_file(&file_path, field).await.unwrap();
                None
            }
            Some(Property::Date(prop)) => {
                let text = field.text().await.unwrap();
                match prop.input_type {
                    DateInputType::Date => Some(Value::String(
                        chrono::NaiveDate::parse_from_str(&text, "%Y-%m-%d")
                            .unwrap()
                            .to_string(),
                    )),
                    DateInputType::DatetimeLocal => Some(Value::String(
                        chrono::NaiveDateTime::parse_from_str(&text, "%Y-%m-%dT%H:%M")
                            .unwrap()
                            .to_string(),
                    )),
                    DateInputType::Time => {
                        parse_time(&text).map(|time| Value::String(time.to_string()))
                    }
                    DateInputType::Month => {
                        parse_month(&text).map(|date| Value::String(date.to_string()))
                    }
                    DateInputType::Week => {
                        parse_week(&text).map(|date| Value::String(date.to_string()))
                    }
                }
            }
            Some(Property::Captcha(_)) => Some(Value::String(field.text().await.unwrap())),
            None => None,
        };
        if let Some(content) = content {
            body.insert(name, content);
        }
    }
    (Value::Object(body), files)
}

fn parse_month(value: &str) -> Option<NaiveDate> {
    let parts = value.split('-').collect::<Vec<_>>();
    if parts.len() != 2 {
        return None;
    }
    let year = parts.first().unwrap().parse::<i32>().unwrap();
    let month = parts.get(1).unwrap().parse::<u32>().unwrap();
    if !(1..=12).contains(&month) {
        return None;
    }
    Some(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
}

fn parse_time(value: &str) -> Option<NaiveTime> {
    let parts = value.split(':').collect::<Vec<_>>();
    if parts.len() != 2 {
        return None;
    }
    let hour = parts.first().unwrap().parse::<u32>().unwrap();
    let minute = parts.get(1).unwrap().parse::<u32>().unwrap();
    if !(0..=23).contains(&hour) || !(0..=59).contains(&minute) {
        return None;
    }
    Some(NaiveTime::from_hms_opt(hour, minute, 0).unwrap())
}

fn parse_week(value: &str) -> Option<NaiveDate> {
    let parts = value.split('-').collect::<Vec<_>>();
    if parts.len() != 2 {
        return None;
    }
    let year = parts.first().unwrap().parse::<i32>().unwrap();
    let week_part = parts.get(1).unwrap().trim_start_matches('W');
    let week = week_part.parse::<i32>().unwrap();
    if !(1..=53).contains(&week) {
        return None;
    }
    let first_day = chrono::NaiveDate::from_ymd_opt(year, 1, 1)?;
    let first_weekday = first_day.weekday();
    let days_offset = first_weekday.num_days_from_monday();
    let first_monday_in_the_year = first_day + chrono::Duration::days(7 - days_offset as i64);
    Some(first_monday_in_the_year + chrono::Duration::weeks(week as i64 - 1))
}

fn parse_number_value(
    value: &str,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
) -> Option<Value> {
    if let Ok(number) = value.parse::<i64>() {
        let number = number.clamp(
            min.unwrap_or(f64::MIN) as i64,
            max.unwrap_or(f64::MAX) as i64,
        );
        // make sure that the number fits into the steps between min and max
        let number = {
            let temp = number - min.unwrap_or(f64::MIN) as i64;
            let temp = temp - (temp % step.unwrap_or(1.0) as i64);
            temp + min.unwrap_or(f64::MIN) as i64
        };
        Some(Value::Number(number.into()))
    } else if let Ok(number) = value.parse::<f64>() {
        let number = number.clamp(min.unwrap_or(f64::MIN), max.unwrap_or(f64::MAX));
        // make sure that the number fits into the steps between min and max
        let number = {
            let temp = number - min.unwrap_or(f64::MIN);
            let temp = if let Some(step) = step {
                temp - (temp % step)
            } else {
                temp
            };
            temp + min.unwrap_or(f64::MIN)
        };
        Some(Value::Number(serde_json::Number::from_f64(number).unwrap()))
    } else {
        None
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
