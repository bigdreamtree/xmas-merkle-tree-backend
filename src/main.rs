mod db;
mod routes;
mod utils;

use axum::{routing::get, routing::post, Router};
use db::connection::create_connection_pool;
use dotenv::dotenv;
use std::{net::SocketAddr, sync::Arc};
use crate::routes::trees::{create_tree_route, get_tree_messages_route, create_tree_message_route};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    // Diesel Connection Manager
    let pool = create_connection_pool();

    let app = Router::new()
        .route("/v1/trees", post(create_tree_route))
        .route("/v1/trees/:account_hash/messages", get(get_tree_messages_route))
        .route("/v1/trees/:account_hash/messages", post(create_tree_message_route))
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}