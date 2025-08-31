use crate::error;
use crate::helpers::app_state::AppState;
use crate::models::{JWTClaims, SellerInvoiceNote};
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
};
use sqlx::query_as;
use std::sync::Arc;

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<JWTClaims>,
) -> Result<Json<Vec<SellerInvoiceNote>>, (StatusCode, String)> {
    let invoice_notes: Vec<SellerInvoiceNote> =
        query_as("SELECT * FROM seller_invoice_notes WHERE seller_id = $1 ORDER BY note")
            .bind(&claims.sub)
            .fetch_all(&app_state.db_pool)
            .await
            .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    Ok(Json(invoice_notes))
}
