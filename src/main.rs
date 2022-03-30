mod handlers;
mod utils;

use axum::{
    routing::{get, get_service},
    Router,
};
use hyper::StatusCode;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ping", get(handlers::ping))
        .nest(
            "/uploads",
            get_service(ServeDir::new("uploads")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .route("/", get(handlers::get_files).post(handlers::accept_form));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(utils::shutdown_signal())
        .await
        .unwrap();
}
