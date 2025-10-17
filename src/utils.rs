use axum::{BoxError, body::Bytes};
use futures::{Stream, TryStreamExt};
use std::{io, path::PathBuf};
use tokio::fs::File;
use tokio_util::io::StreamReader;

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
