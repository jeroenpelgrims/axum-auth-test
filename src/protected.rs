use axum::{
    middleware::from_extractor,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::config::auth::RequireAuth;

pub fn router() -> Router {
    Router::new()
        .route("/", get(protected))
        .route_layer(from_extractor::<RequireAuth>())
}

async fn protected(RequireAuth(claims): RequireAuth) -> impl IntoResponse {
    Html(format!(
        r#"
        <h1>Protected page!</h1>
        <p>Hello, {}</p>
        <a href="/">Back to home</a>
        "#,
        claims.user.name
    ))
}
