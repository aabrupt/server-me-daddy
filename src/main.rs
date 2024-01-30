use std::net::SocketAddr;

use axum::{routing::get, Router};
use templates::Index;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod templates;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .nest_service("/assets", ServeDir::new("./assets"))
        .route("/", get(index_page));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4050));

    println!("Listening to address: http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn index_page() -> Index {
    Index::new("Hello world!")
}
