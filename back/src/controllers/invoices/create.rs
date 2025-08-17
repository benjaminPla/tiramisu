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
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Body {
    buyer_id: Uuid,
    currency: String,
    due_date: i64,
    issue_date: i64,
}

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<JWTClaims>,
    Json(body): Json<Body>,
) -> Result<StatusCode, (StatusCode, String)> {
    query(
        "
        INSERT INTO invoices (
            buyer_id,
            currency,
            due_date,
            issue_date,
            seller_id
        )
        VALUES ($1, $2, to_timestamp($3), to_timestamp($4), $5)
    ",
    )
    .bind(&body.buyer_id)
    .bind(&body.currency)
    .bind(&body.due_date)
    .bind(&body.issue_date)
    .bind(&claims.sub)
    .execute(&app_state.db_pool)
    .await
    .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    Ok(StatusCode::CREATED)
}
