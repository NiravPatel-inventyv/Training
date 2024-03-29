use axum::{routing::get, Router};

pub fn get_health_check() -> Router {
    Router::new().route("/health/check", get(root))
}

// basic handler that responds with a static string
async fn root() -> String {
    "Hello, World!".to_string()
}
