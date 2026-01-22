use axum::{
    Json, Router,
    extract::{Query, State},
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::json;
use ts_rs::TS;

use crate::{
    AppState, database,
    model::{CMSSettings, Form, Settings, StatusResponse},
    render::render_form,
};

pub fn form_router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/forms",
            get(list_forms)
                .post(create_form)
                .put(update_form)
                .delete(delete_form),
        )
        .route("/api/forms/preview", post(preview_form))
        .route("/api/form-select/list", get(list_forms_for_select))
        .route("/api/form-select/by-id", get(get_form_by_id_for_select))
}

#[derive(Debug, Deserialize)]
struct SelectFormQuery {
    value: i32,
}
async fn get_form_by_id_for_select(
    State(state): State<AppState>,
    Query(query): Query<SelectFormQuery>,
) -> impl IntoResponse {
    // we need to cast the id to an int with `id::int`, because otherwise we randomly get an error,
    // that there is not enough data left in message or the binary data in the parameter 1 is
    // incorrect, even though the id column is defined as an `int` which according to the
    // documentation of sqlx, is mapped to an i32
    let form: Form = sqlx::query_as(r#"select * from form where id::int = $1"#)
        .bind(query.value)
        .fetch_one(&state.db)
        .await
        .unwrap();
    Json(form)
}

async fn list_forms_for_select(State(state): State<AppState>) -> impl IntoResponse {
    let forms: Vec<Form> = sqlx::query_as(r#"select * from form"#)
        .fetch_all(&state.db)
        .await
        .unwrap();
    Json(
        forms
            .into_iter()
            .map(|form| {
                json!({
                    "value": form.id.to_string(),
                    "label": form.name,
                })
            })
            .collect::<Vec<_>>(),
    )
}

async fn preview_form(State(state): State<AppState>, Json(form): Json<Form>) -> impl IntoResponse {
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
        br#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
    <style type="text/tailwindcss">
    @custom-variant dark (&:where(.dark, .dark *));
    </style>
    <title>Document</title>
</head>
<body>"#,
        br#"
</body>
</html>"#,
        settings,
    );
    Json(json!({
        "html": rendered
    }))
}

async fn create_form(State(state): State<AppState>, Json(body): Json<Form>) -> Json<Form> {
    let created = database::create_form(body, &state.db).await.unwrap();
    Json(created)
}

async fn update_form(
    State(state): State<AppState>,
    Json(body): Json<Form>,
) -> Json<StatusResponse> {
    database::update_form(body.id, body, &state.db)
        .await
        .unwrap();
    Json(StatusResponse::success())
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
struct DeleteFormBody {
    id: i32,
}
async fn delete_form(
    State(state): State<AppState>,
    Json(body): Json<DeleteFormBody>,
) -> Json<StatusResponse> {
    sqlx::query(r#"delete from form where id = $1"#)
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
