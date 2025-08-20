use crate::error;
use crate::helpers::app_state::AppState;
use crate::models::JWTClaims;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, Validation};
use std::sync::Arc;

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let jar = CookieJar::from_headers(req.headers());
    let token = jar
        .get("token")
        .ok_or_else(|| error!(StatusCode::UNAUTHORIZED))?
        .value();

    let claims = decode::<JWTClaims>(
        &token,
        &DecodingKey::from_secret(&app_state.env_vars.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|e| {
        let message = match e.kind() {
            ErrorKind::ExpiredSignature => "Expired token".to_string(),
            _ => "Invalid token".to_string(),
        };
        error!(StatusCode::UNAUTHORIZED, message)
    })?
    .claims;

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
