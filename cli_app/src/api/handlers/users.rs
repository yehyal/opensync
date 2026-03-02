use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    api::{
        errors::AppError,
        request::login::LoginRequest,
        response::{TokenClaims, login::LoginResponse, users::SingleUserResponse},
    },
    services::user::{CreateUserDTO, UserServiceImpl},
    state::ApplicationState,
};

pub async fn register(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreateUserDTO>,
) -> Result<Json<SingleUserResponse>, AppError> {
    let user = state.user_service.create_user(payload).await?;

    let response = SingleUserResponse { data: user.clone() };
    Ok(Json(response))
}

pub async fn login(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let _user = match state.user_service.get_user_by_name(&payload.username).await {
        Ok(user) => user,
        Err(e) => {
            return Err(AppError::from((
                StatusCode::UNAUTHORIZED,
                anyhow::anyhow!("Invalid username or password"),
            )));
        }
    };

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims = TokenClaims {
        sub: payload.username,
        iat,
        exp,
    };

    let secret = state
        .settings
        .load()
        .token_secret
        .clone()
        .unwrap_or("Secret".to_string());

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    let response = LoginResponse {
        status: "success".to_string(),
        token,
    };

    Ok(Json(response))
}
