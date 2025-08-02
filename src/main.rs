mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

use handlers::upload::upload_pdf_handler;

#[tokio::main]
async fn main() {
    // Build your app with root and upload route
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/upload", post(upload_pdf_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "Hello, Axum without TcpStream!"
}
