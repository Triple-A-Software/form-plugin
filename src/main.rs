use std::{
    collections::HashMap,
    env,
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use chrono::NaiveDate;
use chrono::{Datelike, NaiveTime};
use form_plugin::{
    AppState,
    api::{
        dashboard::{dashboard_card_recent_submissions, dashboard_card_submission_stats},
        form::*,
        settings::*,
        submission::*,
    },
    captcha::CaptchaType,
    clamav::{ScanResult, scan_file},
    create_cms_db,
    model::{
        CMSSettings, DateInputType, EmailAttachment, Property, SelectDataType, SubmissionFile,
    },
    rate_limit::{SubmissionRateLimiter, client_ip},
    render::HONEYPOT_FIELD,
    utils::{
        AppError, AppResult, ErrorReport, get_file_path, get_file_storage_path, stream_to_file,
    },
};
use form_plugin::{api::email::*, render::render_form};
use form_plugin::{captcha::Captcha, extract_field_as_string, send_email};
use form_plugin::{
    cleverreach::{Cleverreach, ReceiverRequest},
    create_db,
};
use form_plugin::{
    database::create_submission,
    model::{CleverreachFormCondition, Form, Settings, Submission},
};
use minijinja::path_loader;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::{
    normalize_path::NormalizePathLayer,
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::Level;
use uuid::Uuid;

/// upper bound for one whole submission; single files are limited per field (see `FileProperty`)
const MAX_SUBMISSION_BYTES: usize = 250 * 1024 * 1024;
const MAX_FILES_PER_SUBMISSION: usize = 20;
const SUBMISSIONS_PER_IP_PER_HOUR: usize = 10;
const UPLOAD_BYTES_PER_IP_PER_HOUR: u64 = 500 * 1024 * 1024;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let db = create_db().await;
    let cms_db = create_cms_db().await;

    // this query needs the do update part, because otherwise we would need another query to fetch
    // the settings, because insert on conflict do nothing only returns something when inserting,
    // not when doing nothing
    let settings: Settings = sqlx::query_as(
        r#"insert into settings (id) values ('settings') on conflict (id) do update set id = 'settings' returning *"#,
    )
    .fetch_one(&db)
    .await
    .unwrap();

    let mut environment = minijinja::Environment::new();
    environment.set_loader(path_loader("./templates"));
    // let tera = Tera::new("./templates/*.html").expect("Failed to parse template files");

    let state = AppState {
        db,
        cms_db,
        environment,
        cleverreach: Arc::new(RwLock::new(Cleverreach::new(
            settings.cleverreach_client_id,
            settings.cleverreach_secret,
        ))),
        rate_limiter: Arc::new(SubmissionRateLimiter::new(
            Duration::from_secs(60 * 60),
            SUBMISSIONS_PER_IP_PER_HOUR,
            UPLOAD_BYTES_PER_IP_PER_HOUR,
        )),
    };
    let router = Router::new()
        .nest_service("/ui", ServeDir::new("ui/dist"))
        .route(
            "/api/forms",
            get(route_list_forms)
                .post(route_create_form)
                .put(route_update_form)
                .delete(route_delete_form),
        )
        .route("/api/forms/preview", post(route_preview_form))
        .route("/api/form-select/list", get(route_list_forms_for_select))
        .route(
            "/api/form-select/by-id",
            get(route_get_form_by_id_for_select),
        )
        .route(
            "/api/submissions",
            get(route_list_submissions).delete(route_delete_submission),
        )
        .route(
            "/api/submissions/send-email",
            post(route_send_email_to_submission),
        )
        .route("/api/submissions/count", get(route_count_submissions))
        .route("/api/submissions/recent", get(route_recent_submissions))
        .route("/api/submissions/stats", get(route_submission_stats))
        .route("/api/submissions/export", get(route_export_submissions))
        .route("/api/submissions/download-files", get(route_download_files))
        .route(
            "/api/settings",
            get(route_get_settings).put(route_update_settings),
        )
        .route(
            "/dashboard/submission-stats",
            get(dashboard_card_submission_stats),
        )
        .route(
            "/dashboard/recent-submissions",
            get(dashboard_card_recent_submissions),
        )
        .route(
            "/api/emails",
            get(route_list_emails)
                .post(route_create_email)
                .put(route_update_email)
                .delete(route_delete_email),
        )
        .route("/helper/render_form", post(render_form_helper))
        .route(
            "/form-plugin/submit",
            post(form_submit).layer(DefaultBodyLimit::max(MAX_SUBMISSION_BYTES)),
        )
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
async fn test_page<'src>(
    State(state): State<AppState<'src>>,
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
    let cms_settings: CMSSettings =
        sqlx::query_as(r#"select * from settings where id = 'settings'"#)
            .fetch_one(&state.cms_db)
            .await
            .unwrap();
    let rendered = render_form(
        state,
        cms_settings.default_language,
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

async fn render_form_helper<'src>(
    State(state): State<AppState<'src>>,
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
            if let Ok(settings) =
                sqlx::query_as::<_, Settings>(r#"select * from settings where id = 'settings'"#)
                    .fetch_one(&state.db)
                    .await
                && let Ok(cms_settings) = sqlx::query_as::<_, CMSSettings>(
                    r#"select * from settings where id = 'settings'"#,
                )
                .fetch_one(&state.cms_db)
                .await
            {
                let rendered = render_form(
                    state,
                    body.page.language.unwrap_or(cms_settings.default_language),
                    form,
                    format!(
                        r#"<form action="/form-plugin/submit?id={id}&redirect={route}" method="POST" enctype="multipart/form-data">"#,
                    ),
                    "</form>",
                    settings,
                );
                Json(ServicePluginApiResponse::Data(Value::String(rendered)))
            } else {
                tracing::error!("Error when loading settings or cms settings from database");
                Json(ServicePluginApiResponse::Error(
                    "internal server error".to_string(),
                ))
            }
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
async fn form_submit<'src>(
    State(state): State<AppState<'src>>,
    Query(query): Query<FormSubmitQuery>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let form: Form = match sqlx::query_as(r#"select * from form where id = $1"#)
        .bind(query.id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(form)) => form,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to load form {}: {e}", query.id);
            return redirect_with_error(
                &headers,
                &query,
                "server_error",
                StatusCode::INTERNAL_SERVER_ERROR,
            );
        }
    };

    let content_length = headers
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(MAX_SUBMISSION_BYTES as u64);
    // without a forwarded ip (local dev) there is no per-client key, so we don't limit
    if let Some(ip) = client_ip(&headers)
        && !state.rate_limiter.check(ip, content_length, Instant::now())
    {
        tracing::warn!("Rate limited submission for form {} from {ip}", form.id);
        return redirect_with_error(
            &headers,
            &query,
            "rate_limited",
            StatusCode::TOO_MANY_REQUESTS,
        );
    }

    let settings: Settings = match sqlx::query_as(r#"select * from settings where id = 'settings'"#)
        .fetch_one(&state.db)
        .await
    {
        Ok(settings) => settings,
        Err(e) => {
            tracing::error!("Failed to load settings: {e}");
            return redirect_with_error(
                &headers,
                &query,
                "server_error",
                StatusCode::INTERNAL_SERVER_ERROR,
            );
        }
    };

    // every early return below must delete the uploaded files, nothing may stay on disk unless
    // the submission is stored
    let submission_id = Uuid::new_v4();
    let mut files = Vec::new();
    let (body, honeypot_filled) = match extract_form_body(
        &mut multipart,
        &form,
        &submission_id,
        settings.captcha_provider,
        &mut files,
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!("Rejected submission for form {}: {e}", form.id);
            delete_files(&files).await;
            return redirect_with_error(
                &headers,
                &query,
                "upload_rejected",
                StatusCode::BAD_REQUEST,
            );
        }
    };

    if honeypot_filled {
        tracing::info!(
            "Discarded submission for form {} (honeypot filled)",
            form.id
        );
        delete_files(&files).await;
        // look like a success so bots don't learn about the trap
        return redirect_or_ok(&query.redirect);
    }

    if form.activate_captcha {
        let captcha_response = match settings.captcha_provider {
            Some(r#type) => {
                Captcha {
                    body: &body,
                    r#type: &r#type,
                }
                .handle(settings.clone())
                .await
            }
            None => None,
        };
        let error = match captcha_response {
            Some(response) if response.success => None,
            Some(response) => {
                tracing::warn!(
                    "Captcha failed for form {}: {:?}",
                    form.id,
                    response.error_codes
                );
                Some("captcha_error")
            }
            None => Some("captcha_missing"),
        };
        if let Some(error) = error {
            delete_files(&files).await;
            return redirect_with_error(&headers, &query, error, StatusCode::UNAUTHORIZED);
        }
    }

    if let Some(address) = settings
        .clamav_address
        .as_deref()
        .map(str::trim)
        .filter(|a| !a.is_empty())
    {
        for file in &files {
            // fail closed: only an explicit clean verdict lets the submission through
            let rejected = match scan_file(address, &get_file_path(&file.id)).await {
                Ok(ScanResult::Clean) => false,
                Ok(ScanResult::Infected(signature)) => {
                    tracing::warn!(
                        "Blocked upload {:?} for form {}: {signature}",
                        file.name,
                        form.id
                    );
                    true
                }
                Err(e) => {
                    tracing::error!(
                        "Virus scan failed for upload {:?} for form {}: {e}",
                        file.name,
                        form.id
                    );
                    true
                }
            };
            if rejected {
                delete_files(&files).await;
                return redirect_with_error(
                    &headers,
                    &query,
                    "file_rejected",
                    StatusCode::UNPROCESSABLE_ENTITY,
                );
            }
        }
    }

    let stored: AppResult<()> = async {
        let mut tx = state.db.begin().await?;

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
                    let _ = state
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
                        .report_err(file!(), line!());
                }
            }
        }

        if form.send_submission_as_email_enabled
            && let (Some(submission_receiver_email), Some(submissions_email_id)) =
                (&form.send_submission_as_email, &form.submissions_email_id)
        {
            let attachments = form.email_attach_files.then(|| {
                files
                    .iter()
                    .map(|f| EmailAttachment {
                        name: f.name.clone().unwrap_or_default(),
                        path: get_file_path(&f.id),
                        mime_type: f.mime_type.clone(),
                    })
                    .collect()
            });
            let _ = send_email(
                &mut tx,
                *submissions_email_id,
                &form,
                &body,
                submission_receiver_email,
                attachments,
            )
            .await
            .report_err(file!(), line!());
        }

        if form.send_confirmation_email {
            let confirmation_email_recipient = extract_field_as_string(
                &body,
                form.confirmation_email_recipient_field_name.as_deref(),
            );
            let recipient_field_missing = form
                .confirmation_email_recipient_field_name
                .is_some()
                && confirmation_email_recipient.is_none();

            if let (Some(recipient), Some(confirmation_email_id)) =
                (confirmation_email_recipient, &form.confirmation_email_id)
            {
                let _ = send_email(
                    &mut tx,
                    *confirmation_email_id,
                    &form,
                    &body,
                    &recipient,
                    None,
                )
                .await
                .report_err(file!(), line!());
            } else if recipient_field_missing {
                tracing::warn!(
                    "Confirmation email enabled but recipient field '{}' not found in submission data",
                    form.confirmation_email_recipient_field_name.as_deref().unwrap()
                );
            }
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
        .await?;
        for file in &files {
            sqlx::query(
                r#"
                    insert into submission_file (id, name, submission_id, mime_type)
                    values ($1, $2, $3, $4) returning *
                "#,
            )
            .bind(file.id)
            .bind(&file.name)
            .bind(submission_id)
            .bind(&file.mime_type)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }
    .await;

    if let Err(e) = stored {
        tracing::error!("Failed to store submission for form {}: {e}", form.id);
        delete_files(&files).await;
        return redirect_with_error(
            &headers,
            &query,
            "server_error",
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    }
    redirect_or_ok(&query.redirect)
}

fn redirect_or_ok(redirect: &Option<String>) -> Response {
    match redirect {
        Some(redirect) => Redirect::to(redirect).into_response(),
        None => StatusCode::OK.into_response(),
    }
}

/// Sends the visitor back to the page with the form (taken from the `Referer`) with `error` and
/// `form` query params, so `render_form` can show the message there. Falls back to the redirect
/// target, or answers with `status` if neither is known.
fn redirect_with_error(
    headers: &HeaderMap,
    query: &FormSubmitQuery,
    error: &str,
    status: StatusCode,
) -> Response {
    let referer = headers.get(header::REFERER).and_then(|v| v.to_str().ok());
    match error_redirect_target(referer, query.redirect.as_deref(), query.id, error) {
        Some(target) => Redirect::to(&target).into_response(),
        None => status.into_response(),
    }
}

fn error_redirect_target(
    referer: Option<&str>,
    redirect: Option<&str>,
    form_id: i32,
    error: &str,
) -> Option<String> {
    if let Some(mut url) = referer.and_then(|r| Url::parse(r).ok()) {
        // drop the params of an earlier failed attempt
        let params: Vec<(String, String)> = url
            .query_pairs()
            .filter(|(k, _)| k != "error" && k != "form")
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        url.query_pairs_mut()
            .clear()
            .extend_pairs(params)
            .append_pair("error", error)
            .append_pair("form", &form_id.to_string());
        // path and query only, so we never redirect to another host
        return Some(format!(
            "{}?{}",
            url.path(),
            url.query().unwrap_or_default()
        ));
    }
    let redirect = redirect?;
    Some(
        match Url::parse_with_params(
            redirect,
            &[("error", error), ("form", &form_id.to_string())],
        ) {
            Ok(url) => url.to_string(),
            // relative redirect targets can't be parsed as a Url
            Err(_) => {
                let separator = if redirect.contains('?') { '&' } else { '?' };
                format!("{redirect}{separator}error={error}&form={form_id}")
            }
        },
    )
}

async fn delete_files(files: &[SubmissionFile]) {
    for file in files {
        if let Err(e) = tokio::fs::remove_file(get_file_path(&file.id)).await
            && e.kind() != std::io::ErrorKind::NotFound
        {
            tracing::error!("Failed to delete rejected upload {}: {e}", file.id);
        }
    }
}

/// Parses the submission. Uploaded files are streamed to disk and pushed onto `files` before they
/// are written, so the caller can clean them up when this fails halfway. Returns the field values
/// and whether the honeypot field was filled in.
async fn extract_form_body(
    multipart: &mut Multipart,
    form: &Form,
    submission_id: &Uuid,
    captcha_provider: Option<CaptchaType>,
    files: &mut Vec<SubmissionFile>,
) -> AppResult<(Value, bool)> {
    let mut body = serde_json::Map::new();
    let mut honeypot_filled = false;
    while let Some(field) = multipart.next_field().await? {
        let Some(name) = field.name().map(str::to_string) else {
            continue;
        };
        if name == HONEYPOT_FIELD {
            honeypot_filled |= !field.text().await?.trim().is_empty();
            continue;
        }
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
            Property::Captcha(_) => match captcha_provider {
                Some(CaptchaType::HCaptcha) => name == "h-captcha-response",
                Some(CaptchaType::ReCaptcha) => name == "g-recaptcha-response",
                Some(CaptchaType::Turnstile) => name == "cf-turnstile-response",
                None => false,
            },
        });
        let content = match form_field {
            Some(Property::Textarea(_)) | Some(Property::Text(_)) => {
                Some(Value::String(field.text().await?))
            }
            Some(Property::Checkbox(_)) => {
                let text = field.text().await?;
                Some(Value::Bool(text == "on"))
            }
            Some(Property::Number(prop)) => {
                let text = field.text().await?;
                parse_number_value(&text, prop.min, prop.max, prop.step)
            }
            Some(Property::Select(prop)) => match prop.data_type {
                SelectDataType::String => Some(Value::String(field.text().await?)),
                SelectDataType::Number => {
                    parse_number_value(&field.text().await?, None, None, None)
                }
            },
            Some(Property::Range(prop)) => {
                let text = field.text().await?;
                parse_number_value(&text, Some(prop.min), Some(prop.max), prop.step)
            }
            Some(Property::Buttongroup(prop)) => {
                let text = field.text().await?;
                prop.options
                    .iter()
                    .find(|&o| o.value == text)
                    .map(|value| Value::String(value.value.clone()))
            }
            Some(Property::Button(_)) => None,
            Some(Property::File(prop)) => {
                let file_name = field.file_name().unwrap_or_default().to_string();
                // browsers send an empty part for file inputs that were left empty
                if file_name.is_empty() {
                    continue;
                }
                if files.len() >= MAX_FILES_PER_SUBMISSION {
                    return Err(AppError::BadRequest("too many files".to_string()));
                }
                let mime_type = field
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .to_string();
                if !prop.accepts(&file_name, &mime_type) {
                    return Err(AppError::BadRequest(format!(
                        "file type not accepted: {file_name} ({mime_type})"
                    )));
                }
                tokio::fs::create_dir_all(get_file_storage_path()).await?;
                let file_id = Uuid::new_v4();
                files.push(SubmissionFile {
                    id: file_id,
                    name: Some(file_name),
                    submission_id: *submission_id,
                    mime_type,
                });
                stream_to_file(&get_file_path(&file_id), field, prop.max_file_size_bytes()).await?;
                None
            }
            Some(Property::Date(prop)) => {
                let text = field.text().await?;
                match prop.input_type {
                    DateInputType::Date => chrono::NaiveDate::parse_from_str(&text, "%Y-%m-%d")
                        .ok()
                        .map(|date| Value::String(date.to_string())),
                    DateInputType::DatetimeLocal => {
                        chrono::NaiveDateTime::parse_from_str(&text, "%Y-%m-%dT%H:%M")
                            .ok()
                            .map(|date| Value::String(date.to_string()))
                    }
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
            Some(Property::Captcha(_)) => Some(Value::String(field.text().await?)),
            None => None,
        };
        if let Some(content) = content {
            body.insert(name, content);
        }
    }
    Ok((Value::Object(body), honeypot_filled))
}

fn parse_month(value: &str) -> Option<NaiveDate> {
    let parts = value.split('-').collect::<Vec<_>>();
    if parts.len() != 2 {
        return None;
    }
    let year = parts.first()?.parse::<i32>().ok()?;
    let month = parts.get(1)?.parse::<u32>().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    NaiveDate::from_ymd_opt(year, month, 1)
}

fn parse_time(value: &str) -> Option<NaiveTime> {
    let parts = value.split(':').collect::<Vec<_>>();
    if parts.len() != 2 {
        return None;
    }
    let hour = parts.first()?.parse::<u32>().ok()?;
    let minute = parts.get(1)?.parse::<u32>().ok()?;
    if !(0..=23).contains(&hour) || !(0..=59).contains(&minute) {
        return None;
    }
    NaiveTime::from_hms_opt(hour, minute, 0)
}

fn parse_week(value: &str) -> Option<NaiveDate> {
    let parts = value.split('-').collect::<Vec<_>>();
    if parts.len() != 2 {
        return None;
    }
    let year = parts.first()?.parse::<i32>().ok()?;
    let week_part = parts.get(1)?.trim_start_matches('W');
    let week = week_part.parse::<i32>().ok()?;
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
        serde_json::Number::from_f64(number).map(Value::Number)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_redirect_goes_back_to_the_form_page() {
        assert_eq!(
            error_redirect_target(
                Some("https://lr3d.de/anfrage?lang=de&error=captcha_error&form=1#top"),
                Some("/danke"),
                3,
                "file_rejected"
            )
            .as_deref(),
            Some("/anfrage?lang=de&error=file_rejected&form=3")
        );
        assert_eq!(
            error_redirect_target(Some("https://evil.example/x"), None, 3, "rate_limited")
                .as_deref(),
            Some("/x?error=rate_limited&form=3"),
            "never leaves the host"
        );
        assert_eq!(
            error_redirect_target(None, Some("/danke?a=1"), 3, "upload_rejected").as_deref(),
            Some("/danke?a=1&error=upload_rejected&form=3")
        );
        assert_eq!(
            error_redirect_target(None, Some("https://lr3d.de/danke"), 3, "upload_rejected")
                .as_deref(),
            Some("https://lr3d.de/danke?error=upload_rejected&form=3")
        );
        assert_eq!(
            error_redirect_target(None, None, 3, "upload_rejected"),
            None
        );
    }
}
