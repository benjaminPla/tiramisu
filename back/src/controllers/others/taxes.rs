use crate::error;
use crate::helpers::app_state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::{query_as};
use std::sync::Arc;
use crate::models::Tax;

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Tax>>, (StatusCode, String)> {
    let taxes: Vec<Tax> = query_as("SELECT * FROM taxes")
        .fetch_all(&app_state.db_pool)
        .await
        .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    Ok(Json(taxes))
}
