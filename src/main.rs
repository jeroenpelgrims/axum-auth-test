use auth::User;
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

    // let session_layer = SessionManagerLayer::new(MemoryStore::default());
    // let user_id = uuid::Uuid::new_v4();
    // let backend = Backend {
    //     users: vec![(
    //         user_id,
    //         User {
    //             id: user_id,
    //             name: "Joske Vermeulen".to_string(),
    //             username: "user".to_string(),
    //             password: "pass".to_string(),
    //         },
    //     )]
    //     .into_iter()
    //     .collect(),
    // };
    // let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(login))
        .route("/login", post(login_post))
        .route("/logout", get(logout))
        .nest("/protected", protected::router())
        // .layer(auth_layer)
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

async fn login_post() -> impl IntoResponse {
    // let user = match auth_session.authenticate(creds.clone()).await {
    //     Ok(Some(user)) => user,
    //     Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
    //     Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    // };

    // auth_session.login(&user).await.unwrap();

    // Redirect::to("/protected").into_response()

    let max_age = time::Duration::hours(1);
    let token = auth::create_token("<user_id>".to_string(), max_age).unwrap();
    let cookie = auth::create_token_cookie(token, max_age);

    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, cookie.to_string())
        .body(Body::empty())
        .unwrap()
}

async fn logout() -> impl IntoResponse {
    let cookie = auth::create_token_cookie("".to_owned(), Duration::hours(-1));

    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, cookie.to_string())
        .body(Body::empty())
        .unwrap()
}
