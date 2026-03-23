use std::{
    env::current_dir,
    fs::create_dir_all,
    io::Error,
    pin::pin,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    body::{Body, Bytes},
    extract::Request,
    response::Response,
};
use futures_util::{Stream, TryStreamExt};
use http::{StatusCode, header};
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::{ReaderStream, StreamReader};
use tower_http::BoxError;

use crate::api::errors::AppError;
const UPLOADS_DIRECTORY: &str = "uploads";
pub async fn download() -> Result<Response, AppError> {
    print!("DIRECTORY {}", current_dir().unwrap().display());
    let stream = File::open("big.bin").await?;
    let metadata = &stream.metadata().await?;
    let tokio = ReaderStream::new(stream);
    let body = Body::from_stream(tokio);

    let response: Response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(header::CONTENT_LENGTH, metadata.len())
        .body(body)?;

    Ok(response)
}

pub async fn upload(request: Request) -> Result<(), AppError> {
    stream_to_file(request.into_body().into_data_stream()).await
}

async fn stream_to_file<S, E>(stream: S) -> Result<(), AppError>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    async {
        let body_with_err = stream.map_err(Error::other);
        let mut body_reader = pin!(StreamReader::new(body_with_err));

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let file_name = format!("output_file_{}", timestamp);
        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(UPLOADS_DIRECTORY).join(file_name);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;
        Ok::<_, std::io::Error>(())
    }
    .await
    .map_err(|e| AppError::from(e))
}
