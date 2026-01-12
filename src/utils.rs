use axum::{BoxError, body::Bytes};
use futures::{Stream, TryStreamExt};
use std::{env, fmt::Display, io, path::PathBuf};
use tokio::fs::File;
use tokio_util::io::StreamReader;
use uuid::Uuid;

// from https://github.com/tokio-rs/axum/blob/main/examples/stream-to-file/src/main.rs
// Save a `Stream` to a file
pub async fn stream_to_file<S, E>(path: &PathBuf, stream: S) -> anyhow::Result<()>
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
