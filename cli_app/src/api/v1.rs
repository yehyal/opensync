use super::handlers;
use crate::{api::middleware::auth::auth, state::ApplicationState};
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
use std::sync::Arc;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route("/test", get(handlers::test::test))
        .route(
            "/login",
            post(handlers::users::login).with_state(state.clone()),
        )
        .route(
            "/posts",
            get(handlers::posts::get_all).with_state(state.clone()),
        )
        .route(
            "/posts",
            post(handlers::posts::create)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route(
            "/posts/{id}",
            put(handlers::posts::update).with_state(state.clone()),
        )
        .route(
            "/posts/{id}",
            get(handlers::posts::get).with_state(state.clone()),
        )
        .route(
            "/posts/{id}",
            delete(handlers::posts::delete).with_state(state),
        )
}
