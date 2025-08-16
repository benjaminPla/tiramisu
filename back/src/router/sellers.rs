use crate::controllers::sellers::create;
use crate::helpers::app_state::AppState;
use axum::{routing::post, Router};
use std::sync::Arc;

pub struct SellersRouter;

impl SellersRouter {
    pub fn new() -> Router<Arc<AppState>> {
        Router::new().route("/create", post(create::handler))
    }
}
