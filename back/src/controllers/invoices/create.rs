use crate::error;
use crate::helpers::app_state::AppState;
use crate::models::{JWTClaims};
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use sqlx::query_scalar;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct BodyDetails {
    description: String,
    quantity: i32,
    tax_id: i32,
    unit_price: f64
}

#[derive(Deserialize)]
pub struct Body {
    buyer_id: Uuid,
    currency: String,
    details: Vec<BodyDetails>,
    due_date: i64,
    issue_date: i64,
}

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<JWTClaims>,
    Json(body): Json<Body>,
) -> Result<StatusCode, (StatusCode, String)> {
    if body.details.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invoice must have at least one detail".to_string(),
        ));
    }

    let mut tx = app_state
        .db_pool
        .begin()
        .await
        .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    let invoice_id: Uuid = query_scalar(
        "
        INSERT INTO invoices (buyer_id, currency, due_date, issue_date, seller_id)
        VALUES ($1, $2, to_timestamp($3), to_timestamp($4), $5)
        RETURNING id
        ",
    )
    .bind(&body.buyer_id)
    .bind(&body.currency)
    .bind(&body.due_date)
    .bind(&body.issue_date)
    .bind(&claims.sub)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    for detail in &body.details {
        sqlx::query(
            "
            INSERT INTO invoice_details
            (description, invoice_id, quantity, seller_id, tax_id, unit_price)
            VALUES ($1, $2, $3, $4, $5, $6)
            ",
        )
        .bind(&detail.description)
        .bind(&invoice_id)
        .bind(&detail.quantity)
        .bind(&claims.sub)
        .bind(&detail.tax_id)
        .bind(&detail.unit_price)
        .execute(&mut *tx)
        .await
        .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;
    }

    tx.commit()
        .await
        .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    Ok(StatusCode::CREATED)
}
