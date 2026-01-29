use axum::{
    Router,
    routing::{get, put},
};

use crate::http::{object_handler, root_handler};

pub mod http;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler::handle()))
        .route("/object/:bucket/*key", put(object_handler::put_object))
        // .route("/object/:bucket/*key", get(object_handler::))
            ;

    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listner, app).await.unwrap();
}
