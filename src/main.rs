mod routes;
mod models;
mod utils;

use axum::{routing::get, routing::post, Router};
use std::net::SocketAddr;
use crate::routes::trees::{create_tree, get_tree_messages, create_tree_message};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    let app = Router::new()
        .route("/v1/trees", post(create_tree))
        .route("/v1/trees/:account_hash/messages", get(get_tree_messages))
        .route("/v1/trees/:account_hash/messages", post(create_tree_message));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}