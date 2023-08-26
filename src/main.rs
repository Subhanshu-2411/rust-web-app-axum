#![allow(unused)]

use std::net::SocketAddr;
use axum::{Router, ServiceExt};
use axum::routing::get;
use axum::response::Html;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!!!</string>") }),
    );

    // region:       ----Start Server

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("--> Listening on {address}\n");

    axum::Server::bind(&address)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();

    // endregion:    ----Start Server
}