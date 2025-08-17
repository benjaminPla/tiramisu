use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

#[derive(Clone, Deserialize, Serialize)]
pub struct JWTClaims {
    pub exp: usize,
    pub sub: i32,
}

#[derive(FromRow)]
pub struct Seller {
    address: String,
    bank_account: String,
    city: String,
    email: String,
    pub hashed_password: String,
    pub id: i32,
    name: String,
    postal_code: String,
    public_id: Uuid,
    vat_number: String,
}
