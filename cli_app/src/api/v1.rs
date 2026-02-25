use std::sync::Arc;

use crate::state::ApplicationState;

use super::handlers;
use axum::{Router, routing::get};

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route("/hello", get(handlers::test::test))
        .with_state(state)
}
