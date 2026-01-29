use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;
use ts_rs::TS;

use crate::{
    AppState, database,
    model::{CMSSettings, Form, Settings, StatusResponse},
    render::render_form,
};

#[derive(Debug, Deserialize)]
pub struct SelectFormQuery {
    value: i32,
}
pub async fn route_get_form_by_id_for_select<'src>(
    State(state): State<AppState<'src>>,
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

pub async fn route_list_forms_for_select<'src>(
    State(state): State<AppState<'src>>,
) -> impl IntoResponse {
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

pub async fn route_preview_form<'src>(
    State(state): State<AppState<'src>>,
    Json(form): Json<Form>,
) -> impl IntoResponse {
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

pub async fn route_create_form<'src>(
    State(state): State<AppState<'src>>,
    Json(body): Json<Form>,
) -> Json<Form> {
    let created = database::create_form(body, &state.db).await.unwrap();
    Json(created)
}

pub async fn route_update_form<'src>(
    State(state): State<AppState<'src>>,
    Json(body): Json<Form>,
) -> Json<StatusResponse> {
    database::update_form(body.id, body, &state.db)
        .await
        .unwrap();
    Json(StatusResponse::success())
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
pub struct DeleteFormBody {
    id: i32,
}
pub async fn route_delete_form<'src>(
    State(state): State<AppState<'src>>,
    Json(body): Json<DeleteFormBody>,
) -> Json<StatusResponse> {
    sqlx::query(r#"delete from form where id = $1"#)
        .bind(body.id)
        .execute(&state.db)
        .await
        .unwrap();
    Json(StatusResponse::success())
}

pub async fn route_list_forms<'src>(State(state): State<AppState<'src>>) -> Json<Vec<Form>> {
    Json(
        sqlx::query_as(r#"select * from form"#)
            .fetch_all(&state.db)
            .await
            .unwrap(),
    )
}
