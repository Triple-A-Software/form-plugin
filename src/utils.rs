use axum::{BoxError, Json, body::Bytes, extract::multipart::MultipartError, http, response::IntoResponse};
use futures::{Stream, TryStreamExt};
use serde::Serialize;
use std::{env, fmt, fmt::Display, io, path::PathBuf};
use tokio::fs::File;
use tokio_util::io::StreamReader;
use uuid::Uuid;

#[derive(Debug)]
pub enum AppError {
    Anyhow(anyhow::Error),
    BadRequest(String),
    NotFound(String),
    Csv(csv::Error),
    Infallible(std::convert::Infallible),
    Http(http::Error),
    HeaderValue(http::header::InvalidHeaderValue),
    Io(io::Error),
    Multipart(MultipartError),
    Reqwest(reqwest::Error),
    Sqlx(sqlx::Error),
    Template(minijinja::Error),
    TempFile(async_tempfile::Error),
    Utf8(std::string::FromUtf8Error),
    Zip(async_zip::error::ZipError),
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Anyhow(err) => write!(f, "{err}"),
            AppError::BadRequest(message) => write!(f, "{message}"),
            AppError::NotFound(message) => write!(f, "{message}"),
            AppError::Csv(err) => write!(f, "{err}"),
            AppError::Infallible(err) => write!(f, "{err}"),
            AppError::Http(err) => write!(f, "{err}"),
            AppError::HeaderValue(err) => write!(f, "{err}"),
            AppError::Io(err) => write!(f, "{err}"),
            AppError::Multipart(err) => write!(f, "{err}"),
            AppError::Reqwest(err) => write!(f, "{err}"),
            AppError::Sqlx(err) => write!(f, "{err}"),
            AppError::Template(err) => write!(f, "{err}"),
            AppError::TempFile(err) => write!(f, "{err}"),
            AppError::Utf8(err) => write!(f, "{err}"),
            AppError::Zip(err) => write!(f, "{err}"),
        }
    }
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

impl From<csv::Error> for AppError {
    fn from(err: csv::Error) -> Self {
        AppError::Csv(err)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Anyhow(err)
    }
}

impl From<std::convert::Infallible> for AppError {
    fn from(err: std::convert::Infallible) -> Self {
        AppError::Infallible(err)
    }
}

impl From<http::Error> for AppError {
    fn from(err: http::Error) -> Self {
        AppError::Http(err)
    }
}

impl From<http::header::InvalidHeaderValue> for AppError {
    fn from(err: http::header::InvalidHeaderValue) -> Self {
        AppError::HeaderValue(err)
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<MultipartError> for AppError {
    fn from(err: MultipartError) -> Self {
        AppError::Multipart(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Reqwest(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Sqlx(err)
    }
}

impl From<minijinja::Error> for AppError {
    fn from(err: minijinja::Error) -> Self {
        AppError::Template(err)
    }
}

impl From<async_tempfile::Error> for AppError {
    fn from(err: async_tempfile::Error) -> Self {
        AppError::TempFile(err)
    }
}

impl From<std::string::FromUtf8Error> for AppError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        AppError::Utf8(err)
    }
}

impl From<async_zip::error::ZipError> for AppError {
    fn from(err: async_zip::error::ZipError) -> Self {
        AppError::Zip(err)
    }
}

// from https://github.com/tokio-rs/axum/blob/main/examples/stream-to-file/src/main.rs
// Save a `Stream` to a file
pub async fn stream_to_file<S, E>(path: &PathBuf, stream: S) -> AppResult<()>
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

    // Copy the body into the file.
    tokio::io::copy(&mut body_reader, &mut file).await?;

    Ok(())
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
