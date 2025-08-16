use crate::error;
use crate::helpers::app_state::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Deserialize, Serialize)]
pub struct JWTClaims {
    pub exp: usize,
    pub sub: String,
}

pub async fn handler(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let token = headers
        .get("authorization")
        .ok_or(error!(StatusCode::UNAUTHORIZED))?
        .to_str()
        .map_err(|e| error!(StatusCode::UNAUTHORIZED, err:e))?;

    if token.is_empty() {
        return Err(error!(
            StatusCode::UNAUTHORIZED,
            "Missing token".to_string()
        ));
    }

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
