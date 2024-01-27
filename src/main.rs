use std::net::SocketAddr;

use api;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(index_page));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4050));

    println!("Listening to address: {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn index_page() {}
