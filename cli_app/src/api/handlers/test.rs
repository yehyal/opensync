use std::env::current_dir;

use axum::{body::Body, response::Response};
use http::{StatusCode, header};
use tokio::fs::File;
use tokio_util::io;

use crate::api::errors::AppError;

pub async fn test() -> Result<Response, AppError> {
    print!("DIRECTORY {}", current_dir().unwrap().display());
    let stream = File::open("big.bin").await?;
    let metadata = &stream.metadata().await?;
    let tokio = io::ReaderStream::new(stream);
    let body = Body::from_stream(tokio);

    let response: Response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(header::CONTENT_LENGTH, metadata.len())
        .body(body)?;

    Ok(response)
}
