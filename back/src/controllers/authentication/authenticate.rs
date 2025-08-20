use crate::error;
use crate::helpers::app_state::AppState;
use crate::models::{JWTClaims, Seller};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use axum::{extract::State, http::StatusCode, response::Json};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Deserialize;
use sqlx::query_as;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Body {
    email: String,
    password: String,
}

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(body): Json<Body>,
) -> Result<(CookieJar, StatusCode), (StatusCode, String)> {
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
        sub: seller.id,
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(app_state.env_vars.jwt_secret.as_bytes()),
    )
    .map_err(|e| error!(StatusCode::INTERNAL_SERVER_ERROR, err: e))?;

    let cookie = Cookie::build(("token", token.clone()))
        .http_only(true)
        .max_age(cookie::time::Duration::hours(1))
        .path("/")
        .same_site(SameSite::Lax)
        .secure(true)
        .build();

    let updated_jar = jar.add(cookie);

    Ok((updated_jar, StatusCode::OK))
}
