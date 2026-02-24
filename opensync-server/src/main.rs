use anyhow::{Context, Ok};
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(hello))
        .layer(tower_http::catch_panic::CatchPanicLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("Failed to bind to tcp listener")?;
    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;

    Ok(())
}

async fn hello() -> (StatusCode, Json<Response>) {
    let response = Response {
        message: "Hello world",
    };

    (StatusCode::OK, Json(response))
}
