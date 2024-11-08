mod db;
mod routes;
mod utils;

use axum::{routing::get, routing::post, Router};
use db::connection::create_connection_pool;
use dotenv::dotenv;
use routes::messages::{create_tree_message_route, get_tree_messages_reveal_route, get_tree_messages_route};
use std::{net::SocketAddr, sync::Arc};
use crate::routes::trees::create_tree_route;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // env check
    let _ = std::env::var("PINATA_JWT").expect("PINATA_JWT must be set");
    let _ = std::env::var("PINATA_GROUP_ID").expect("PINATA_GROUP_ID must be set");
    let _ = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    // Diesel Connection Manager
    let pool = create_connection_pool();

    let app = Router::new()
        .route("/v1/trees", post(create_tree_route))
        .route("/v1/trees/:account_hash/messages", get(get_tree_messages_route))
        .route("/v1/trees/:account_hash/messages", post(create_tree_message_route))
        .route("/v1/trees/:account_hash/messages/reveal", post(get_tree_messages_reveal_route))
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(pool));

    let port: u16 = std::env::var("PORT").unwrap_or("3000".to_string()).parse().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}