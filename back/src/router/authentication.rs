use crate::controllers::authentication::authenticate;
use crate::helpers::app_state::AppState;
use axum::{routing::post, Router};
use std::sync::Arc;

pub struct AuthenticationRouter;

impl AuthenticationRouter {
    pub fn new() -> Router<Arc<AppState>> {
        Router::new().route("/authenticate", post(authenticate::handler))
    }
}
