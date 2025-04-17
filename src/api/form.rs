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
    AppState,
    model::{Form, StatusResponse},
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
        .route("/api/form-select", get(list_forms_for_select))
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
                    "value": form.id,
                    "label": form.name,
                })
            })
            .collect::<Vec<_>>(),
    )
}

async fn preview_form(State(state): State<AppState>, Json(form): Json<Form>) -> impl IntoResponse {
    let rendered = render_form(
        state,
        "en",
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
    );
    Json(json!({
        "html": rendered
    }))
}

async fn create_form(State(state): State<AppState>, Json(body): Json<Form>) -> Json<Form> {
    let created = sqlx::query_as(
        r#"insert into form (
            name,
            send_confirmation_email,
            confirmation_email_id,
            confirmation_email_recipient_field_name,
            props
        ) values ($1, $2, $3, $4, $5) returning *"#,
    )
    .bind(body.name)
    .bind(body.send_confirmation_email)
    .bind(body.confirmation_email_id)
    .bind(body.confirmation_email_recipient_field_name)
    .bind(body.props)
    .fetch_one(&state.db)
    .await
    .unwrap();
    Json(created)
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

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
struct DeleteFormQuery {
    id: i32,
}
async fn delete_form(
    State(state): State<AppState>,
    Query(query): Query<DeleteFormQuery>,
) -> Json<StatusResponse> {
    sqlx::query(r#"delete from form where id = $1"#)
        .bind(query.id)
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
