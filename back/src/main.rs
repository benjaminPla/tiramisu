mod controllers;
mod helpers;
mod macros;
mod middlewares;
mod models;
mod router;

use helpers::app_state::AppState;
use router::MainRouter;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app_state = AppState::new().await.expect("Error creating AppState");
    let app = MainRouter::new(&app_state);
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Error binding to port 3000");
    axum::serve(listener, app)
        .await
        .expect("Error starting server");
}
