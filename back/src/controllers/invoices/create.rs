use crate::error;
use crate::helpers::app_state::AppState;
use crate::models::{Invoice, InvoiceDetail, JWTClaims, SellerNote};
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Error as SqlxError};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Body {
    buyer_id: Uuid,
    currency: String,
    details: Vec<BodyDetails>,
    due_date: NaiveDate,
    issue_date: NaiveDate,
    notes: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct BodyDetails {
    description: String,
    quantity: i32,
    tax_id: i32,
    unit_price: f64,
}

#[derive(Serialize)]
pub struct Res {
    invoice: Invoice,
    invoice_details: Vec<InvoiceDetail>,
    invoice_notes: Vec<SellerNote>,
}

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<JWTClaims>,
    Json(body): Json<Body>,
) -> Result<Json<Res>, (StatusCode, String)> {
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

    let invoice: Invoice = query_as(
        "
        INSERT INTO invoices (buyer_id, currency, due_date, issue_date, seller_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
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

    let mut invoice_details = Vec::with_capacity(body.details.len());
    for detail in &body.details {
        let invoice_detail: InvoiceDetail = query_as(
            "
            INSERT INTO invoice_details
            (description, invoice_id, quantity, seller_id, tax_id, unit_price)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            ",
        )
        .bind(&detail.description)
        .bind(&invoice.id)
        .bind(&detail.quantity)
        .bind(&claims.sub)
        .bind(&detail.tax_id)
        .bind(&detail.unit_price)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

        invoice_details.push(invoice_detail);
    }

    let mut invoice_notes = Vec::with_capacity(body.notes.len());
    for note_id in &body.notes {
        let seller_note: SellerNote = query_as("SELECT * FROM seller_invoice_notes WHERE id = $1;")
            .bind(&note_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| match e {
                SqlxError::RowNotFound => error!(StatusCode::NOT_FOUND, "Note not found"),
                _ => error!(StatusCode::INTERNAL_SERVER_ERROR, err: e),
            })?;

        query(
            "
            INSERT INTO invoice_notes
            (invoice_id, note_id)
            VALUES ($1, $2)
            RETURNING *
            ",
        )
        .bind(&invoice.id)
        .bind(&note_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err:e))?;

        invoice_notes.push(seller_note);
    }

    tx.commit()
        .await
        .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    Ok(Json(Res {
        invoice,
        invoice_details,
        invoice_notes,
    }))
}
