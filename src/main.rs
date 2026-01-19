mod models;
mod storage;
mod handler;
mod utils;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let app = Router::new()
        .route("/", get(|| async { "SOLUX System Active" }))
        .route("/tap/:id", get(handler::profile_handler))
        .nest_service("/static", ServeDir::new("static"));

    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
    
    println!("--- SOLUX SIMULATION STARTED ---");
    println!("Server running at: http://{}", addr);
    println!("Simulated QR for 'alex':\n{}", utils::generate_qr_string("http://localhost:3000/tap/alex"));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
