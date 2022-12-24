use anyhow::Result;
use axum::{Router, Server};

async fn handle_error(err: std::io::Error) -> (http::StatusCode, String) {
    (
        http::StatusCode::NOT_FOUND,
        format!("File not found: {err}"),
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    let static_service = axum::error_handling::HandleError::new(
        tower_http::services::ServeDir::new("./static"),
        handle_error,
    );

    let app = Router::new().nest_service("/static/", static_service);

    Ok(Server::bind(&"0.0.0.0:8000".parse()?)
        .serve(app.into_make_service())
        .await?)
}