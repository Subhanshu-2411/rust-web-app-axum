#![allow(unused)]

use axum::Router;
use axum::routing::get;
use axum::response::Html;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!!!</string>") }),
    );
}