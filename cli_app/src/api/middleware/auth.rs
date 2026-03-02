use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{
    api::{errors::AppError, response::TokenClaims},
    state::ApplicationState,
};

pub async fn auth(
    State(state): State<Arc<ApplicationState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| auth_value.strip_prefix("Bearer ").map(|s| s.to_owned()));
    let token = token.ok_or_else(|| {
        AppError::from((
            StatusCode::UNAUTHORIZED,
            anyhow::anyhow!("Missing Bearer Token"),
        ))
    })?;

    let secret = state
        .settings
        .load()
        .token_secret
        .clone()
        .unwrap_or("Secret".to_string());

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| {
        AppError::from((
            StatusCode::UNAUTHORIZED,
            anyhow::anyhow!("Invalid bearer token"),
        ))
    })?
    .claims;

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
