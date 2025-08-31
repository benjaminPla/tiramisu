use crate::controllers::sellers::{create_invoice_note, update};
use crate::helpers::app_state::AppState;
use crate::middlewares::with_auth;
use axum::{
    middleware::from_fn_with_state,
    routing::{post, put},
    Router,
};
use std::sync::Arc;

pub struct SellersRouter;

impl SellersRouter {
    pub fn new(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
        Router::new()
            .route(
                "/create_invoice_note",
                post(create_invoice_note::handler)
                    .layer(from_fn_with_state(app_state.clone(), with_auth::handler)),
            )
            .route(
                "/update",
                put(update::handler)
                    .layer(from_fn_with_state(app_state.clone(), with_auth::handler)),
            )
    }
}
