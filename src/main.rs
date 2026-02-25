use axum::{
    Router,
    routing::{get, put},
};
use object_store_rust::http::{app_state::AppState, config::Config, object_handler, root_handler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let db = sled::open("my_db").expect("not able to open my_db");
    let config = Config::new(
        std::env::var("OBJECT_STORE_SEGMENT_PATH").expect("OBJECT_STORE_SEGMENT_PATH required"),
        std::env::var("OBJECT_STORE_STANDALONE_PATH")
            .expect("OBJECT_STORE_STANDALONE_PATH required"),
    );
    let app_state = AppState::new(db, config);

    let app = Router::new()
        .route("/", get(root_handler::handle()))
        .route("/object/{bucket}/{*key}", put(object_handler::put_object))
        .route("/object/{bucket}/{*key}", get(object_handler::get_object))
        .route("/object/{bucket}", get(object_handler::list_object))
        .with_state(app_state);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listner, app).await.unwrap();
}
