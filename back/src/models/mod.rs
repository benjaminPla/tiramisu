use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, FromRow, Serialize)]
pub struct Country(String);

#[derive(Deserialize)]
pub struct InvoiceDetail {
    description: String,
    id: Uuid,
    invoice_id: Uuid,
    quantity: i32,
    seller_id: Uuid,
    tax_id: i32,
    unit_price: f64,
}

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
