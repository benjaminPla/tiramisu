use crate::error;
use crate::helpers::app_state::AppState;
use crate::models::JWTClaims;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Body {
    address: String,
    bank_account: String,
    city: String,
    country: String,
    email: String,
    name: String,
    postal_code: String,
    vat_number: String,
}

#[derive(FromRow, Serialize)]
pub struct Res {
    address: String,
    bank_account: String,
    city: String,
    country: String,
    email: String,
    name: String,
    postal_code: String,
    vat_number: String,
}

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<JWTClaims>,
    Json(body): Json<Body>,
) -> Result<Json<Res>, (StatusCode, String)> {
    let updated_seller: Option<Res> = query_as(
        "
        UPDATE sellers SET
            address = $1,
            bank_account = $2,
            city = $3,
            country = $4,
            email = $5,
            name = $6,
            postal_code = $7,
            vat_number = $8
        WHERE id = $9
        RETURNING id, address, bank_account, city, country, email, name, postal_code, vat_number
    ",
    )
    .bind(&body.address)
    .bind(&body.bank_account)
    .bind(&body.city)
    .bind(&body.country)
    .bind(&body.email)
    .bind(&body.name)
    .bind(&body.postal_code)
    .bind(&body.vat_number)
    .bind(&claims.sub)
    .fetch_optional(&app_state.db_pool)
    .await
    .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    match updated_seller {
        Some(seller) => Ok(Json(seller)),
        None => Err(error!(StatusCode::NOT_FOUND)),
    }
}
