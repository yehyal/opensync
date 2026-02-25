use std::sync::Arc;

use axum::{extract::State, http::StatusCode};

use crate::state::ApplicationState;

pub async fn test(State(state): State<Arc<ApplicationState>>) -> Result<String, StatusCode> {
    Ok(format!(
        "\n Hello World, Configuration from {} \n\n",
        state
            .settings
            .load()
            .config
            .location
            .clone()
            .unwrap_or("[nowhere]".to_string())
    ))
}
