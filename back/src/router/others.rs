use crate::controllers::others::{countries, currencies, taxes};
use crate::helpers::app_state::AppState;
use axum::{http::StatusCode, routing::get, Router};
use std::sync::Arc;

pub struct OthersRouter;

impl OthersRouter {
    pub fn new(_app_state: Arc<AppState>) -> Router<Arc<AppState>> {
        Router::new()
            .route("/countries", get(countries::handler))
            .route("/currencies", get(currencies::handler))
            .route("/health", get(|| async { StatusCode::OK }))
            .route("/taxes", get(taxes::handler))
    }
}
