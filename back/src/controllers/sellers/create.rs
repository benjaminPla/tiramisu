use crate::helpers::app_state::AppState;
use crate::http_error;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract::State, http::StatusCode, response::Json};
use serde::Deserialize;
use sqlx::query;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Body {
    address: String,
    bank_account: String,
    city: String,
    country: String,
    email: String,
    password: String,
    name: String,
    postal_code: String,
    vat_number: String,
}

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<Body>,
) -> Result<StatusCode, (StatusCode, String)> {
    let argon2 = Argon2::default();
    let hashed_password = argon2
        .hash_password(&body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map_err(|e| http_error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?
        .to_string();

    query(
        "
        INSERT INTO sellers (
            address,
            bank_account,
            city,
            country,
            email,
            hashed_password,
            name,
            postal_code,
            vat_number
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    ",
    )
    .bind(&body.address)
    .bind(&body.bank_account)
    .bind(&body.city)
    .bind(&body.country)
    .bind(&body.email)
    .bind(&hashed_password)
    .bind(&body.name)
    .bind(&body.postal_code)
    .bind(&body.vat_number)
    .execute(&app_state.db_pool)
    .await
    .map_err(|e| http_error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    Ok(StatusCode::CREATED)
}
