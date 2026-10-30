use axum::{BoxError, Json, body::Bytes, extract::multipart::MultipartError, http, response::IntoResponse};
use futures::{Stream, TryStreamExt};
use serde::Serialize;
use std::{env, fmt::Display, io, path::PathBuf};
use tokio::{fs::File, io::AsyncReadExt};
use tokio_util::io::StreamReader;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
    #[error(transparent)]
    Http(#[from] http::Error),
    #[error(transparent)]
    HeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Multipart(#[from] MultipartError),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Template(#[from] minijinja::Error),
    #[error(transparent)]
    TempFile(#[from] async_tempfile::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Zip(#[from] async_zip::error::ZipError),
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AppError::BadRequest(_) | AppError::Multipart(_) | AppError::HeaderValue(_) => {
                http::StatusCode::BAD_REQUEST
            }
            AppError::NotFound(_) => http::StatusCode::NOT_FOUND,
            _ => http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Json(ErrorResponse {
            error: self.to_string(),
        });
        (status, body).into_response()
    }
}

// from https://github.com/tokio-rs/axum/blob/main/examples/stream-to-file/src/main.rs
// Save a `Stream` to a file, failing with `BadRequest` once it grows beyond `max_bytes`
pub async fn stream_to_file<S, E>(path: &PathBuf, stream: S, max_bytes: u64) -> AppResult<u64>
where
    S: Stream<Item = Result<Bytes, E>> + Unpin,
    E: Into<BoxError>,
{
    // Convert the stream into an `AsyncRead`.
    let body_with_io_error = stream.map_err(io::Error::other);
    let body_reader = StreamReader::new(body_with_io_error);
    futures::pin_mut!(body_reader);

    // Create the file. `File` implements `AsyncWrite`.
    let mut file = File::create(path).await?;

    // Copy the body into the file, reading one byte past the limit to detect oversized files.
    let written = tokio::io::copy(&mut body_reader.take(max_bytes + 1), &mut file).await?;
    if written > max_bytes {
        return Err(AppError::BadRequest("file too large".to_string()));
    }

    Ok(written)
}

pub fn get_file_storage_path() -> PathBuf {
    PathBuf::from(env::var("FILE_STORAGE_PATH").unwrap_or("/var/form-plugin".to_string()))
}

pub fn get_file_path(file_id: &Uuid) -> PathBuf {
    get_file_storage_path().join(file_id.to_string())
}

pub trait ErrorReport {
    fn report_err(self, file: impl Display, line: impl Display) -> Self;
}

impl<T, E> ErrorReport for std::result::Result<T, E>
where
    E: Display,
{
    fn report_err(self, file: impl Display, line: impl Display) -> Self {
        if let Err(err) = self.as_ref() {
            tracing::error!("[{}:{}] {}", file, line, err.to_string());
            self
        } else {
            self
        }
    }
}
