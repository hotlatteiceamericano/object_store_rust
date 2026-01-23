use axum::{Router, routing::get};

use crate::http::root_handler;

pub mod http;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root_handler::handle()));
    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listner, app).await.unwrap();
}
