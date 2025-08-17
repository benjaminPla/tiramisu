use crate::controllers::others::{countries, taxes};
use crate::helpers::app_state::AppState;
use axum::{http::StatusCode, routing::get, Router};
use std::sync::Arc;

pub struct OthersRouter;

impl OthersRouter {
    pub fn new(_app_state: Arc<AppState>) -> Router<Arc<AppState>> {
        Router::new()
            .route("/health", get(|| async { StatusCode::OK }))
            .route("/countries", get(countries::handler))
            .route("/taxes", get(taxes::handler))
    }
}
