use sqlx::{FromRow, types::Uuid};

#[derive(FromRow)]
pub struct Seller {
    address: String,
    bank_account: String,
    city: String,
    email: String,
    pub hashed_password: String,
    id: i32,
    name: String,
    postal_code: String,
    pub public_id: Uuid,
    vat_number: String,
}
