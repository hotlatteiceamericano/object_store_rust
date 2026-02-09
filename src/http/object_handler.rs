use std::path::Path;

use anyhow::{Context, anyhow};
use axum::{
    body::Bytes,
    extract::{Path as AxumPath, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use object_store_rust::{
    common::store_type::StoreType,
    store::{
        app_error::AppError, metadata::Metadata, object_store::ObjectStore,
        segment_store::SegmentStore, standalone_store::StandaloneStore,
    },
};

use crate::http::app_state::AppState;

pub const SMALL_OBJECT_SIZE_THRESHOLD: usize = 30 * 1024;

pub async fn put_object(
    State(state): State<AppState>,
    AxumPath((bucket, key)): AxumPath<(String, String)>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let (prefix, filename) = get_prefix_filename(&key);

    println!(
        "received PUT request, bucket: {} prefixe: {}, file name: {}, object size: {}",
        bucket,
        prefix,
        filename,
        body.len()
    );

    let store_type = if body.len() <= SMALL_OBJECT_SIZE_THRESHOLD {
        // SegmentStore is shared in the state, unlike StandaloneStore
        let mut segment_store = SegmentStore::new()?;
        segment_store.save(&body).await

        // todo: save the metadata using the result from store.save
    } else {
        // StandaloneState's instantiate is cheap, can be constructed during the request handlings
        let mut store = StandaloneStore::new();
        store.save(&body).await
    }?;

    let metadata = Metadata::new(&bucket, &prefix, &filename).with_store_type(store_type);
    metadata.save(&state.db)?;

    Ok(format!("successfully save metadata: {:?}", metadata))
}

/// Finds the metadata with bucket, prefix and filename
/// Locate the object store from metadata
pub async fn get_object(
    State(state): State<AppState>,
    AxumPath((bucket, key)): AxumPath<(String, String)>,
) -> Result<Response, AppError> {
    let (prefix, filename) = get_prefix_filename(&key);

    // this is the "error handler"
    let metadata = Metadata::read(&state.db, &bucket, prefix, filename).context(format!(
        "failed to read metadata with bucket: {}, prefix: {}, filename: {}",
        bucket, prefix, filename
    ))?;

    // this is the "null handler"
    // also, one way to convert Options to Results (let else)
    let Some(metadata) = metadata else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    // another way to convert Options to Results (ok_or_else)
    let store_type = metadata
        .store_type()
        .as_ref()
        .ok_or_else(|| anyhow!("no store type ine metadata"))?;

    // todo: return stream based on different store_path
    let path = match store_type {
        StoreType::Packed { .. } => {
            return Err(anyhow::anyhow!("packed storage type not supported yet").into());
        }
        StoreType::Standalone { file_path } => file_path,
    };

    let stream = StandaloneStore::open(path.to_path_buf())
        .await
        .context("failed to open object file")?;

    Ok(axum::body::Body::from_stream(stream).into_response())
}

fn get_prefix_filename(key: &str) -> (&str, &str) {
    let std_path = Path::new(key);
    let prefix = std_path.parent().and_then(|p| p.to_str()).unwrap_or("");
    let filename = std_path
        .file_name()
        .and_then(|p| p.to_str())
        .unwrap_or(&key);

    (prefix, filename)
}

fn format_bytes(bytes: usize) -> String {
    const UNITS: [&str; 6] = ["Bytes", "KB", "MB", "GB", "TB", "PB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let bytes_f64 = bytes as f64;
    let exp = (bytes_f64.ln() / 1024_f64.ln()).floor() as usize;
    let exp = exp.min(UNITS.len() - 1);

    let value = bytes_f64 / 1024_f64.powi(exp as i32);

    format!("{:.2} {}", value, UNITS[exp])
}

#[cfg(test)]
mod test {
    use std::fs;

    use axum::{
        Router,
        body::Bytes,
        routing::{get, put},
    };
    use axum_test::TestServer;
    use rstest::{fixture, rstest};

    use crate::http::{app_state::AppState, object_handler};

    #[fixture]
    fn test_server() -> TestServer {
        let db = sled::Config::new()
            .temporary(true)
            .open()
            .expect("cannot open a temporary db in object_handler::test");
        let app_state = AppState::new(db);
        let app = Router::new()
            .route("/object/{bucket}/{*key}", get(object_handler::get_object))
            .route("/object/{bucket}/{*key}", put(object_handler::put_object))
            .with_state(app_state);

        TestServer::new(app).unwrap()
    }

    #[fixture]
    fn image() -> axum::body::Bytes {
        let image = fs::read("test/test-image.jpeg").unwrap();
        Bytes::from(image)
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_before_put(test_server: TestServer) {
        let response = test_server
            .get("/object/test_bucket/test_prefix/test_filename.txt")
            .await;

        response.assert_status_not_ok();
    }

    #[rstest]
    #[tokio::test]
    async fn test_standalone_put(test_server: TestServer, image: Bytes) {
        let response = test_server
            .put("/object/test_bucket/test_prefix/test_filename.txt")
            .bytes(image)
            .await;

        response.assert_status_ok();
    }

    #[rstest]
    #[tokio::test]
    async fn test_standaloneput_get(test_server: TestServer, image: Bytes) {
        let put_response = test_server
            .put("/object/test_bucket/test_prefix/test_filename.txt")
            .bytes(image)
            .await;

        put_response.assert_status_ok();

        let read_response = test_server
            .get("/object/test_bucket/test_prefix/test_filename.txt")
            .await;

        read_response.assert_status_ok();
    }
}
