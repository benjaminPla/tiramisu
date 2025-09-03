use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, FromRow, Serialize)]
pub struct Buyer {
    address: String,
    city: String,
    country: String,
    email: String,
    id: Uuid,
    name: String,
    postal_code: String,
    seller_id: Uuid,
    vat_number: String,
}

#[derive(Deserialize, FromRow, Serialize)]
pub struct Country(String);

#[derive(Deserialize, FromRow, Serialize)]
pub struct Currency {
    currency: String,
    symbol: String,
}

#[derive(Deserialize, FromRow, Serialize)]
pub struct Invoice {
    buyer_id: Uuid,
    currency: String,
    due_date: NaiveDate,
    pub id: Uuid,
    issue_date: NaiveDate,
    number: i32,
    seller_id: Uuid,
}

#[derive(Deserialize, FromRow, Serialize)]
pub struct InvoiceDetail {
    description: String,
    id: Uuid,
    invoice_id: Uuid,
    quantity: f64,
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

#[derive(Deserialize, FromRow, Serialize)]
pub struct SellerInvoiceNote {
    id: Uuid,
    note: String,
    seller_id: Uuid,
}

#[derive(Deserialize, FromRow, Serialize)]
pub struct Tax {
    id: i32,
    rate: f64,
    tax: String,
}

#[derive(Deserialize, FromRow, Serialize)]
pub struct Vendor {
    address: String,
    id: Uuid,
    name: String,
    seller_id: Uuid,
    vat_number: String,
}
