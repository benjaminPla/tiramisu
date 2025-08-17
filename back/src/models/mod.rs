use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct JWTClaims {
    pub exp: usize,
    pub sub: Uuid,
}

#[derive(FromRow)]
pub struct Seller {
    address: String,
    bank_account: String,
    city: String,
    email: String,
    pub hashed_password: String,
    pub id: Uuid,
    name: String,
    postal_code: String,
    vat_number: String,
}
