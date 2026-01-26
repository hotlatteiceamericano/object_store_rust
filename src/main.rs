use axum::{
    extract::DefaultBodyLimit,
    routing::{get, put},
    Router,
};

use crate::http::{object, root_handler};

pub mod http;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler::handle()))
        .route("/object/:bucket/*key", put(object::put_object))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024 * 1024));

    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listner, app).await.unwrap();
}
