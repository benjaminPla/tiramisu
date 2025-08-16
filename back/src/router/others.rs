use crate::helpers::app_state::AppState;
use axum::{http::StatusCode, routing::get, Router};
use std::sync::Arc;

pub struct OthersRouter;

impl OthersRouter {
    pub fn new() -> Router<Arc<AppState>> {
        Router::new().route("/health", get(|| async { StatusCode::OK }))
    }
}
