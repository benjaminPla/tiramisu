mod others;

use crate::helpers::app_state::AppState;
use axum::{http::HeaderValue, Router};
use others::OthersRouter;
use std::sync::Arc;
use std::time::Duration;
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowOrigin, Any, CorsLayer},
    timeout::TimeoutLayer,
};

pub struct MainRouter;

impl MainRouter {
    pub fn new(app_state: &Arc<AppState>) -> Router {
        Router::new()
            .nest(
                "/api",
                Router::new()
                    .nest("/others", OthersRouter::new())
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
            .allow_origin(AllowOrigin::list([
                "https://memelibre.com".parse::<HeaderValue>().unwrap(),
                "http://memelibre.com".parse::<HeaderValue>().unwrap(),
                "localhost:3000".parse::<HeaderValue>().unwrap(),
            ]))
            .allow_methods(Any)
            .allow_headers(Any)
    }
}
