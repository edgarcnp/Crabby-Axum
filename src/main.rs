#![allow(unused)] // Remove later

use axum::serve;
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber::{self, util::SubscriberInitExt};

mod routes;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Define routes
    let router = routes::app_routes();

    // Start the server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
