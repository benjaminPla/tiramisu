use crate::error;
use crate::helpers::app_state::AppState;
use crate::models::JWTClaims;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use sqlx::query;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Body {
    note: String,
}

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<JWTClaims>,
    Json(body): Json<Body>,
) -> Result<StatusCode, (StatusCode, String)> {
    query(
        "
        INSERT INTO seller_notes (note, seller_id)
        VALUES ($1, $2)
        RETURNING *
        ",
    )
    .bind(&body.note)
    .bind(&claims.sub)
    .execute(&app_state.db_pool)
    .await
    .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    Ok(StatusCode::CREATED)
}
