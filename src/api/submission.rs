use std::collections::HashMap;

use async_tempfile::TempFile;
use async_zip::{ZipEntryBuilder, tokio::write::ZipFileWriter};
use axum::{
    Json,
    body::Body,
    extract::{Query, State},
    http::{HeaderMap, HeaderValue},
    response::{IntoResponse, Response},
};
use reqwest::header;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tokio_util::{compat::FuturesAsyncWriteCompatExt, io::ReaderStream};
use ts_rs::TS;
use uuid::Uuid;

use crate::{
    AppState, convert_to_csv,
    date::JavascriptDate,
    extract_field_as_string,
    model::{Form, StatusResponse, Submission, SubmissionApiResponse, SubmissionFile},
    send_email,
    utils::{AppError, AppResult, get_file_path},
};

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
pub struct DownloadFilesQuery {
    id: Uuid,
}
pub async fn route_download_files<'src>(
    State(state): State<AppState<'src>>,
    Query(query): Query<DownloadFilesQuery>,
) -> AppResult<Response> {
    let files: Vec<SubmissionFile> = sqlx::query_as(
        r#"
            select * from submission_file where submission_id = $1
        "#,
    )
    .bind(query.id)
    .fetch_all(&state.db)
    .await?;
    if files.is_empty() {
        return Err(AppError::NotFound("submission files not found".to_string()));
    }
    if files.len() == 1 {
        let file_path = get_file_path(&files[0].id);
        let file = tokio::fs::File::open(file_path).await?;
        Ok(Response::builder()
            .header("Content-Type", files[0].mime_type.as_str())
            .header(
                "Content-Disposition",
                format!(
                    "attachment; filename=\"{}\"",
                    files[0].name.as_deref().unwrap_or("file")
                ),
            )
            .body(Body::from_stream(ReaderStream::new(file)))
            ?)
    } else {
        let mut zip_file = TempFile::new().await?;
        let mut writer = ZipFileWriter::with_tokio(&mut zip_file);
        for file in files {
            let file_path = get_file_path(&file.id);
            let builder = ZipEntryBuilder::new(
                file.name.unwrap_or_else(|| file.id.to_string()).into(),
                async_zip::Compression::Deflate,
            );
            let entry_writer = writer.write_entry_stream(builder).await?;
            let mut file_reader = tokio::fs::File::open(file_path).await?;
            tokio::io::copy(&mut file_reader, &mut entry_writer.compat_write())
                .await?;
        }
        writer.close().await?;
        zip_file.sync_all().await?;
        let zip_file = tokio::fs::File::open(zip_file.file_path()).await?;
        Ok(Response::builder()
            .header("Content-Type", "application/zip")
            .header(
                "Content-Disposition",
                format!("attachment; filename=\"submission_files_{}.zip\"", query.id),
            )
            .body(Body::from_stream(ReaderStream::new(zip_file)))
            ?)
    }
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "index.ts")]
pub struct SubmissionStats {
    total: i32,
    #[ts(type = "Array<{ date: string } & Record<string, number>>")]
    graph: Vec<ApiGraphValue>,
}
#[derive(Serialize, FromRow)]
struct ApiGraphValue {
    date: JavascriptDate,
    #[serde(flatten)]
    values: HashMap<String, i32>,
}
#[derive(FromRow, Debug)]
struct GraphValue {
    date: JavascriptDate,
    form_id: i32,
    value: i64,
}
pub async fn route_submission_stats<'src>(
    State(state): State<AppState<'src>>,
) -> AppResult<Json<SubmissionStats>> {
    let graph: Vec<GraphValue> = sqlx::query_as(
        r#"
        select date_trunc('day', "timestamp") as date, count(timestamp) as value, form_id 
        from "submission"
        group by date, form_id order by date
        "#,
    )
    .fetch_all(&state.db)
    .await?;
    let total: i64 = sqlx::query_scalar(
        r#"
            select count(*) from "submission"
        "#,
    )
    .fetch_one(&state.db)
    .await?;
    let graph = graph
        .into_iter()
        .fold(Vec::new(), |mut acc: Vec<ApiGraphValue>, g: GraphValue| {
            if let Some(value) = acc.iter_mut().find(|a| a.date == g.date) {
                value.values.insert(g.form_id.to_string(), g.value as i32);
            } else {
                acc.push(ApiGraphValue {
                    date: g.date,
                    values: HashMap::from([(g.form_id.to_string(), g.value as i32)]),
                });
            }
            acc
        });
    Ok(Json(SubmissionStats {
        graph,
        total: total as i32,
    }))
}

pub async fn route_recent_submissions<'src>(
    State(state): State<AppState<'src>>,
) -> AppResult<Json<Vec<Submission>>> {
    let submissions: Vec<Submission> =
        sqlx::query_as(r#"select * from submission order by timestamp desc limit 10"#)
            .fetch_all(&state.db)
            .await?;
    Ok(Json(submissions))
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
pub struct SendEmailToSubmissionBody {
    email_id: i32,
    submission_id: Uuid,
    email_field_name: Option<String>,
}
pub async fn route_send_email_to_submission<'src>(
    State(state): State<AppState<'src>>,
    Json(body): Json<SendEmailToSubmissionBody>,
) -> AppResult<Json<StatusResponse>> {
    let mut tx = state.db.begin().await?;
    let submission: Submission = sqlx::query_as(r#"select * from submission where id = $1"#)
        .bind(body.submission_id)
        .fetch_one(&mut *tx)
        .await?;
    let form: Form = sqlx::query_as(r#"select * from form where id = $1"#)
        .bind(submission.form_id)
        .fetch_one(&mut *tx)
        .await?;
    let email = extract_field_as_string(
        &submission.data,
        Some(body.email_field_name.as_deref().unwrap_or("email")),
    );
    if let Some(email) = email {
        send_email(
            &mut tx,
            body.email_id,
            &form,
            &submission.data,
            &email,
            None,
        )
        .await?;
        tx.commit().await?;
        Ok(Json(StatusResponse::success()))
    } else {
        Ok(Json(StatusResponse::fail()))
    }
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
pub struct ListSubmissionsQuery {
    form_id: Option<i32>,
    page: i32,
    items_per_page: i32,
}
pub async fn route_list_submissions<'src>(
    State(state): State<AppState<'src>>,
    Query(query): Query<ListSubmissionsQuery>,
) -> AppResult<Json<Vec<SubmissionApiResponse>>> {
    if let Some(form_id) = query.form_id {
        Ok(Json(
            sqlx::query_as(
                r#"
                select submission.*, count(submission_file) as file_count from submission
                left join submission_file on submission.id = submission_file.submission_id
                where form_id = $1
                group by submission.id, submission_file.id
                order by timestamp desc limit $2 offset $3
                "#,
            )
            .bind(form_id)
            .bind(query.items_per_page)
            .bind(query.page * query.items_per_page)
            .fetch_all(&state.db)
            .await?,
        ))
    } else {
        Ok(Json(
            sqlx::query_as(
                r#"
                select submission.*, count(submission_file) as file_count from submission
                left join submission_file on submission.id = submission_file.submission_id
                group by submission.id, submission_file.id
                order by timestamp desc limit $1 offset $2
                "#,
            )
            .bind(query.items_per_page)
            .bind(query.page * query.items_per_page)
            .fetch_all(&state.db)
            .await?,
        ))
    }
}

pub async fn route_count_submissions<'src>(
    State(state): State<AppState<'src>>,
    Query(query): Query<ListSubmissionsQuery>,
) -> AppResult<Json<i64>> {
    if let Some(form_id) = query.form_id {
        Ok(Json(
            sqlx::query_scalar(r#"select count(*) from submission where form_id = $1"#)
                .bind(form_id)
                .fetch_one(&state.db)
                .await?,
        ))
    } else {
        Ok(Json(
            sqlx::query_scalar(r#"select count(*) from submission"#)
                .fetch_one(&state.db)
                .await?,
        ))
    }
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
pub struct ExportSubmissionsQuery {
    form_id: Option<i32>,
    start_date: Option<JavascriptDate>,
    end_date: Option<JavascriptDate>,
}

pub async fn route_export_submissions<'src>(
    State(state): State<AppState<'src>>,
    Query(query): Query<ExportSubmissionsQuery>,
) -> AppResult<impl IntoResponse> {
    let submissions: Vec<Submission> = sqlx::query_as(
        r#"
        select * from submission
        where
            (form_id = coalesce($1, form_id) or $1 is null)
            and (timestamp >= coalesce($2, timestamp) or $2 is null)
            and (timestamp <= coalesce($3, timestamp) or $3 is null)
        "#,
    )
    .bind(query.form_id)
    .bind(query.start_date)
    .bind(query.end_date)
    .fetch_all(&state.db)
    .await?;
    let mut headers = HeaderMap::new();
    let date = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S");
    headers.insert("Content-type", HeaderValue::from_static("text/csv"));
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(
            format!("attachment; filename=\"submissions_export_{date}.csv\"").as_str(),
        )
        ?,
    );
    Ok((headers, convert_to_csv(&submissions)?))
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
pub struct DeleteSubmissionBody {
    id: Uuid,
}
pub async fn route_delete_submission<'src>(
    State(state): State<AppState<'src>>,
    Json(body): Json<DeleteSubmissionBody>,
) -> AppResult<Json<StatusResponse>> {
    sqlx::query(r#"delete from submission where id = $1"#)
        .bind(body.id)
        .execute(&state.db)
        .await?;
    Ok(Json(StatusResponse::success()))
}
