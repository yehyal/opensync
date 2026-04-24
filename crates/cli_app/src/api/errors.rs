use axum::response::IntoResponse;
use http::StatusCode;
use tokio::io;

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

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::Error::from(value),
        )
    }
}

impl From<http::Error> for AppError {
    fn from(value: http::Error) -> Self {
        Self(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::Error::from(value),
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.0, format!("{:?}", self.1)).into_response()
    }
}
