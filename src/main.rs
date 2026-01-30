use axum::{
    Router,
    routing::{get, put},
};

use crate::http::{app_state::AppState, object_handler, root_handler};

pub mod http;

#[tokio::main]
async fn main() {
    let db = sled::open("my_db").expect("not able to open my_db");
    let app_state = AppState::new(db);

    let app = Router::new()
        .route("/", get(root_handler::handle()))
        .route("/object/:bucket/*key", put(object_handler::put_object))
        .route("/object/:bucket/*key", get(object_handler::get_object))
        .with_state(app_state);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listner, app).await.unwrap();
}
