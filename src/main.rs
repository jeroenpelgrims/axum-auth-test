use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use dotenv::dotenv;
use tower_livereload::LiveReloadLayer;

mod auth;
mod config;
mod protected;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(index))
        .nest("/auth", routes::auth::router())
        .nest("/protected", protected::router())
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
            <li><a href="/auth/login">login</a></li>
            <li><a href="/auth/logout">logout</a></li>
        </ul>"##,
    )
}
