use axum::{
    Router,
    routing::{get, post},
};

pub fn app_routes() -> Router {
    Router::new().route("/", get(hello_world))
}

async fn hello_world() -> &'static str {
    "Hello, Axum!"
}
