mod helpers;
mod router;

use helpers::app_state::AppState;
use tokio::net::TcpListener;
use router::MainRouter;

#[tokio::main]
async fn main() {
    let app_state = AppState::new().await.expect("Error creating AppState");
    let app = MainRouter::new(&app_state);
    let listener = TcpListener::bind("0.0.0.0:3000").await.expect("Error binding to port 3000");
    axum::serve(listener, app).await.expect("Error starting server");
}

