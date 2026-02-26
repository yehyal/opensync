use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    api::{errors::AppError, response::users::SingleUserResponse},
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
