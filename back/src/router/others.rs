use crate::helpers::app_state::AppState;
use axum::{Router, routing::get, http::StatusCode};
use std::sync::Arc;

pub struct OthersRouter;

impl OthersRouter {
    pub fn new() -> Router<Arc<AppState>> {
        Router::new()
            .route("/health", get(|| async { StatusCode::OK }))
    }
}

