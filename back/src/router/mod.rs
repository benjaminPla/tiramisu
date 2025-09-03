mod authentication;
mod buyers;
mod invoices;
mod others;
mod sellers;
mod vendors;

use crate::helpers::app_state::AppState;
use authentication::AuthenticationRouter;
use axum::{http::HeaderValue, Router};
use buyers::BuyersRouter;
use invoices::InvoicesRouter;
use others::OthersRouter;
use sellers::SellersRouter;
use std::sync::Arc;
use std::time::Duration;
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowOrigin, Any, CorsLayer},
    timeout::TimeoutLayer,
};
use vendors::VendorsRouter;

pub struct MainRouter;

impl MainRouter {
    pub fn new(app_state: &Arc<AppState>) -> Router {
        Router::new()
            .nest(
                "/api",
                Router::new()
                    .nest(
                        "/authentication",
                        AuthenticationRouter::new(app_state.clone()),
                    )
                    .nest("/buyers", BuyersRouter::new(app_state.clone()))
                    .nest("/invoices", InvoicesRouter::new(app_state.clone()))
                    .nest("/others", OthersRouter::new(app_state.clone()))
                    .nest("/sellers", SellersRouter::new(app_state.clone()))
                    .nest("/vendors", VendorsRouter::new(app_state.clone()))
                    .with_state(app_state.clone()),
            )
            .layer(Self::cors())
            .layer(CompressionLayer::new())
            .layer(TimeoutLayer::new(Duration::from_secs(
                app_state.env_vars.timeout_duration,
            )))
    }

    fn cors() -> CorsLayer {
        CorsLayer::new()
            .allow_headers(Any)
            .allow_methods(Any)
            .allow_origin(AllowOrigin::list([
                "https://memelibre.com".parse::<HeaderValue>().unwrap(),
                "http://memelibre.com".parse::<HeaderValue>().unwrap(),
                "localhost:5173".parse::<HeaderValue>().unwrap(),
            ]))
    }
}
