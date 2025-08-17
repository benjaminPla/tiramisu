use crate::error;
use crate::helpers::app_state::AppState;
use crate::models::{Buyer, JWTClaims};
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};
use sqlx::query_as;
use std::sync::Arc;

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<JWTClaims>,
) -> Result<Json<Vec<Buyer>>, (StatusCode, String)> {
    let buyers: Vec<Buyer> = query_as("SELECT * FROM buyers where seller_id = $1")
        .bind(&claims.sub)
        .fetch_all(&app_state.db_pool)
        .await
        .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    Ok(Json(buyers))
}
