use crate::error;
use crate::helpers::app_state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::{query_as};
use std::sync::Arc;
use crate::models::Country;

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Country>>, (StatusCode, String)> {
    let countries: Vec<Country> = query_as("SELECT country FROM countries")
        .fetch_all(&app_state.db_pool)
        .await
        .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    Ok(Json(countries))
}
