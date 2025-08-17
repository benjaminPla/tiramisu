use crate::controllers::authentication::{authenticate, me};
use crate::helpers::app_state::AppState;
use crate::middlewares::with_auth;
use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub struct AuthenticationRouter;

impl AuthenticationRouter {
    pub fn new(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
        Router::new()
            .route("/authenticate", post(authenticate::handler))
            .route(
                "/me",
                get(me::handler).layer(from_fn_with_state(app_state.clone(), with_auth::handler)),
            )
    }
}
