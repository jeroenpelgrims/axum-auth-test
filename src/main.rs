use auth::{Backend, Credentials, User};
use axum::{
    body::Body,
    http::{header, Response, StatusCode},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_login::{
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder, AuthSession,
};
use dotenv::dotenv;
use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};
use std::env;
use time::Duration;
use tower_livereload::LiveReloadLayer;

mod auth;
mod protected;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let session_layer = SessionManagerLayer::new(MemoryStore::default());
    let user_id = uuid::Uuid::new_v4();
    let backend = Backend {
        users: vec![(
            user_id,
            User {
                id: user_id,
                name: "Joske Vermeulen".to_string(),
                username: "user".to_string(),
                password: "pass".to_string(),
            },
        )]
        .into_iter()
        .collect(),
    };
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(login))
        .route("/login", post(login_post))
        .route("/logout", get(logout))
        .nest("/protected", protected::router())
        .layer(auth_layer)
        .layer(LiveReloadLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> impl IntoResponse {
    Html(
        r##"
        <h1>Index</h1>
        <ul>
            <li><a href="/protected">Protected page</a></li>
            <li><a href="/login">login</a></li>
            <li><a href="/logout">logout</a></li>
        </ul>"##,
    )
}

async fn login() -> impl IntoResponse {
    Html(
        r##"
        <h1>Login</h1>

        <form
            method="post"
            target="/login"
            style="display: flex; flex-direction: column; gap: 1rem;"
        >
            <label>
                Username: 
                <input type="text" name="username"/>
            </label>
            <label>
                Password: 
                <input type="password" name="password"/>
            </label>

            <button type="submit">Log in</button>
        </form>

        <a href="/">back</a>
        "##,
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub user: User,
}

fn create_token(
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

fn create_token_cookie<'a>(token: String, max_age: Duration) -> Cookie<'a> {
    Cookie::build(("token", token))
        .path("/")
        .max_age(max_age)
        .same_site(SameSite::Lax)
        .http_only(true)
        .build()
}

async fn login_post(
    mut auth_session: AuthSession<Backend>,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    auth_session.login(&user).await.unwrap();

    Redirect::to("/protected").into_response()

    // let max_age = time::Duration::hours(1);
    // let token = create_token("<user_id>".to_string(), max_age).unwrap();
    // let cookie = create_token_cookie(token, max_age);

    // Response::builder()
    //     .status(StatusCode::SEE_OTHER)
    //     .header(header::LOCATION, "/")
    //     .header(header::SET_COOKIE, cookie.to_string())
    //     .body(Body::empty())
    //     .unwrap()
}

async fn logout(mut auth_session: AuthSession<Backend>) -> impl IntoResponse {
    auth_session.logout().await.unwrap();
    // let cookie = create_token_cookie("".to_owned(), Duration::seconds(0));

    // Response::builder()
    //     .status(StatusCode::SEE_OTHER)
    //     .header(header::LOCATION, "/")
    //     .header(header::SET_COOKIE, cookie.to_string())
    //     .body(Body::empty())
    //     .unwrap()
    Redirect::to("/")
}
