use crate::error;
use crate::helpers::app_state::AppState;
use crate::models::JWTClaims;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize, FromRow, Serialize)]
pub struct Res {
    address: String,
    bank_account: String,
    city: String,
    email: String,
    id: Uuid,
    name: String,
    postal_code: String,
    vat_number: String,
}

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<JWTClaims>,
) -> Result<Json<Res>, (StatusCode, String)> {
    let seller: Res = query_as(
        "
        SELECT address, bank_account, city, country, email, id, name, postal_code, vat_number
        FROM sellers
        WHERE id = $1
        ",
    )
    .bind(&claims.sub)
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    Ok(Json(seller))
}
