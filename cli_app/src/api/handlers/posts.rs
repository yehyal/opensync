use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    api::{
        errors::AppError,
        response::{
            TokenClaims,
            posts::{ListPostsResponse, SinglePostResponse},
        },
    },
    services::post::{CreatePostDTO, PostService, UpdatePostDTO},
    state::ApplicationState,
};

pub async fn get(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.get_post_by_id(id).await;

    match post {
        Ok(post) => {
            let response = SinglePostResponse { data: post };
            Ok(Json(response))
        }
        Err(e) => Err(AppError::from((StatusCode::NOT_FOUND, e))),
    }
}

pub async fn get_all(
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<ListPostsResponse>, AppError> {
    let posts = state.post_service.get_all_posts().await?;

    Ok(Json(ListPostsResponse { data: posts }))
}

pub async fn create(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreatePostDTO>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.create_post(payload).await?;

    let response = SinglePostResponse { data: post };

    Ok(Json(response))
}

pub async fn update(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePostDTO>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.update_post(id, payload).await?;

    let response = SinglePostResponse { data: post };

    Ok(Json(response))
}

pub async fn delete(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
) -> Result<Json<()>, AppError> {
    state.post_service.delete_post(id).await?;

    Ok(Json(()))
}
