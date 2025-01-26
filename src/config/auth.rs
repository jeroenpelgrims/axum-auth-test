use std::env;

use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, HeaderMap, HeaderValue},
    response::Redirect,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub user: User,
}

pub fn create_token(
    user_id: String,
    max_age: time::Duration,
) -> Result<std::string::String, jsonwebtoken::errors::Error> {
    let now = time::OffsetDateTime::now_utc();
    let iat = now.unix_timestamp();
    let exp = (now + max_age).unix_timestamp();
    let claims: TokenClaims = TokenClaims {
        sub: user_id,
        exp,
        iat,
        user: User {
            id: uuid::Uuid::new_v4(),
            name: "Dummy".to_string(),
            password: "passssss".to_string(),
            username: "dummyuser".to_string(),
        },
    };
    let secret = env::var("JWT_SECRET").unwrap();
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &encoding_key)
}

pub fn create_token_cookie<'a>(token: String, max_age: Duration) -> Cookie<'a> {
    Cookie::build(("token", token))
        .path("/")
        .max_age(max_age)
        .same_site(SameSite::Lax)
        .http_only(true)
        .build()
}

fn get_token_from_headers(headers: &HeaderMap<HeaderValue>) -> Option<String> {
    let cookie_header = headers
        .get(header::COOKIE)
        .and_then(|header| header.to_str().ok());
    let cookies =
        cookie_header.map(|header| Cookie::split_parse(header).filter_map(|cookie| cookie.ok()));
    let token_cookie =
        cookies.and_then(|mut cookies| cookies.find(|cookie| cookie.name().eq("token")));
    token_cookie.map(|cookie| cookie.value().to_string())
}

fn decode_token(token: &str) -> Option<TokenClaims> {
    let secret = env::var("JWT_SECRET").unwrap();
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    jsonwebtoken::decode::<TokenClaims>(token, &decoding_key, &Validation::default())
        .ok()
        .map(|result| result.claims)
}

pub struct RequireAuth(pub TokenClaims);

impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = Redirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = get_token_from_headers(&parts.headers);
        let claims = token.and_then(|token| decode_token(&token));

        match claims {
            Some(claims) => Ok(Self(claims)),
            _ => Err(Redirect::to("/auth/login")),
        }
    }
}
