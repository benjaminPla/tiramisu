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
    address: String,
    name: String,
    vat_number: String,
}

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<JWTClaims>,
    Json(body): Json<Body>,
) -> Result<StatusCode, (StatusCode, String)> {
    query(
        "
        INSERT INTO vendors (
            address,
            name,
            seller_id,
            vat_number
        )
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(&body.address)
    .bind(&body.name)
    .bind(&claims.sub)
    .bind(&body.vat_number)
    .execute(&app_state.db_pool)
    .await
    .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    Ok(StatusCode::CREATED)
}

