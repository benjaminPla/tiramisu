use crate::error;
use crate::helpers::app_state::AppState;
use crate::models::Seller;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use axum::{extract::State, http::StatusCode, response::Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Body {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct Res {
    token: String,
}

#[derive(Serialize)]
pub struct JWTClaims {
    sub: String,
    exp: usize,
}

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<Body>,
) -> Result<Json<Res>, (StatusCode, String)> {
    let seller: Seller = query_as("SELECT * FROM sellers WHERE email = $1")
        .bind(&body.email)
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                error!(StatusCode::UNAUTHORIZED, "Invalid email or password")
            }
            _ => error!(StatusCode::INTERNAL_SERVER_ERROR, err:e),
        })?;

    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&seller.hashed_password)
        .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err:e))?;
    argon2
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_err(|_| error!(StatusCode::UNAUTHORIZED, "Invalid email or password"))?;

    let claims = JWTClaims {
        sub: seller.public_id.to_string(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(app_state.env_vars.jwt_secret.as_bytes()),
    )
    .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    Ok(Json(Res { token }))
}
