// use axum::{http::StatusCode, response::IntoResponse};
// use serde::Serialize;

// #[derive(Serialize)]
// pub struct Response {
//     message: &'static str,
// }

// pub struct AppError(anyhow::Error);

// impl From<anyhow::Error> for AppError {
//     fn from(value: anyhow::Error) -> Self {
//         Self(value)
//     }
// }
// impl IntoResponse for AppError {
//     fn into_response(self) -> axum::response::Response {
//         (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
//     }
// }
// pub mod t;
