use axum::{http::StatusCode, response::IntoResponse};

pub struct AppError(StatusCode, anyhow::Error);

impl From<(StatusCode, anyhow::Error)> for AppError {
    fn from((status_code, value): (StatusCode, anyhow::Error)) -> Self {
        Self(status_code, value)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.0, format!("{:?}", self.1)).into_response()
    }
}
