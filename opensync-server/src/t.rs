// use anyhow::{Context, Ok};
// use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
// use serde::Serialize;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let app = Router::new()
//         .route("/", get(hello))
//         .layer(tower_http::catch_panic::CatchPanicLayer::new());

//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
//         .await
//         .context("Failed to bind to tcp listener")?;
//     axum::serve(listener, app)
//         .await
//         .context("axum::serve failed")?;

//     Ok(())
// }

// async fn hello() -> (StatusCode, Json<Response>) {
//     let response = Response {
//         message: "Hello world",
//     };

//     (StatusCode::OK, Json(response))
// }
