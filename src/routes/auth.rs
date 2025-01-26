use crate::config;
use time::Duration;

use axum::{
    body::Body,
    http::{header, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/login", get(login))
        .route("/login", post(login_post))
        .route("/logout", get(logout))
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

async fn login_post() -> impl IntoResponse {
    let max_age = time::Duration::days(30);
    let token = config::auth::create_token("<user_id>".to_string(), max_age).unwrap();
    let cookie = config::auth::create_token_cookie(token, max_age);

    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, cookie.to_string())
        .body(Body::empty())
        .unwrap()
}

async fn logout() -> impl IntoResponse {
    let cookie = config::auth::create_token_cookie("".to_owned(), Duration::hours(-1));

    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, cookie.to_string())
        .body(Body::empty())
        .unwrap()
}
