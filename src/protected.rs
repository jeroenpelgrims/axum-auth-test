use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_login::login_required;

use crate::auth::Backend;

pub fn router() -> Router {
    Router::new()
        .route("/", get(protected))
        .route_layer(login_required!(Backend, login_url = "/login"))
}

async fn protected() -> impl IntoResponse {
    Html(
        r##"
        <h1>Protected page!</h1>
        <a href="/">Back to home</a>
        "##,
    )
}
