use crate::controllers::invoices::create;
use crate::helpers::app_state::AppState;
use crate::middlewares::with_auth;
use axum::middleware::from_fn_with_state;
use axum::{routing::post, Router};
use std::sync::Arc;

pub struct InvoicesRouter;

impl InvoicesRouter {
    pub fn new(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
        Router::new().route(
            "/create",
            post(create::handler).layer(from_fn_with_state(app_state.clone(), with_auth::handler)),
        )
    }
}
