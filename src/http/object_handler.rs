use std::path::Path;

use anyhow::{Context, anyhow};
use axum::{
    Json,
    body::Bytes,
    extract::{Path as AxumPath, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    common::store_type::StoreType,
    http::{app_state::AppState, list_params::ListParams},
    store::{
        app_error::AppError, metadata::Metadata, object_store::ObjectStore,
        segment_store::SegmentStore, standalone_store::StandaloneStore,
    },
};

pub const SMALL_OBJECT_SIZE_THRESHOLD: usize = 30 * 1024;

/// Storing objects with its bucket, prefix and file name
/// It chooses different object store based on the object size
/// Larger object uses StandaloneStore, save the object in standalone file
/// Smaller object uses SegmentStore, append objects together in segment files
pub async fn put_object(
    State(state): State<AppState>,
    AxumPath((bucket, key)): AxumPath<(String, String)>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let (prefix, filename) = get_prefix_filename(&key);

    let store_type = if body.len() <= SMALL_OBJECT_SIZE_THRESHOLD {
        let mut segment_store = SegmentStore::new()?;
        segment_store.save(&body).await
    } else {
        let mut store = StandaloneStore::new();
        store.save(&body).await
    }?;

    let metadata = Metadata::new(
        &bucket,
        &prefix.unwrap_or_default(),
        &filename.unwrap_or_default(),
    )
    .with_store_type(store_type);
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
    let metadata = Metadata::read(
        &state.db,
        &bucket,
        prefix.unwrap_or(""),
        filename.unwrap_or_default(),
    )
    .context(format!(
        "failed to read metadata with bucket: {}, prefix: {}, filename: {}",
        bucket,
        prefix.unwrap_or_default(),
        filename.unwrap_or_default()
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
        .ok_or_else(|| anyhow!("no store type in metadata"))?;

    let stream = match store_type {
        StoreType::Packed {
            segment_file_path,
            offset,
        } => SegmentStore::open(segment_file_path, *offset)
            .await
            .context("SegmentStore cannot open the file")?,
        StoreType::Standalone { file_path } => StandaloneStore::open(file_path)
            .await
            .context("StandaloneStore cannot open the file")?,
    };

    Ok(axum::body::Body::from_stream(stream).into_response())
}

/// Given a bucket and a prefix, returning all the object names
pub async fn list_object(
    State(state): State<AppState>,
    AxumPath(bucket): AxumPath<String>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Metadata>>, AppError> {
    let metadata_list = Metadata::list(
        &state.db,
        &bucket,
        params.prefix.as_deref(),
        params.filename.as_deref(),
    )
    .context("failed to find metadata during list")?;

    Ok(Json(metadata_list))
}

fn get_prefix_filename(key: &str) -> (Option<&str>, Option<&str>) {
    let std_path = Path::new(key);
    let prefix = std_path.parent().and_then(|p| p.to_str());
    let filename = std_path.file_name().and_then(|p| p.to_str());

    (prefix, filename)
}

#[cfg(test)]
mod test {
    use std::{fs, vec};

    use axum::{
        Router,
        body::Bytes,
        routing::{get, put},
    };
    use axum_test::TestServer;
    use rstest::{fixture, rstest};

    use crate::{
        http::{app_state::AppState, object_handler},
        store::metadata::Metadata,
    };

    #[fixture]
    fn test_server() -> TestServer {
        let db = sled::Config::new()
            .temporary(true)
            .open()
            .expect("cannot open a temporary db in object_handler::test");
        let app_state = AppState::new(db);
        // todo: use the app from the main's one
        let app = Router::new()
            .route("/object/{bucket}/{*key}", get(object_handler::get_object))
            .route("/object/{bucket}/{*key}", put(object_handler::put_object))
            .route("/object/{bucket}", get(object_handler::list_object))
            .with_state(app_state);

        TestServer::new(app).unwrap()
    }

    #[fixture]
    fn image() -> axum::body::Bytes {
        let image = fs::read("test/test-image.jpeg").unwrap();
        Bytes::from(image)
    }

    #[fixture]
    fn text() -> axum::body::Bytes {
        let text = fs::read("test/test.txt").unwrap();
        Bytes::from(text)
    }

    #[rstest]
    #[tokio::test]
    async fn test_standalone_get_before_put(test_server: TestServer) {
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
    async fn test_standalone_put_get(test_server: TestServer, image: Bytes) {
        let put_response = test_server
            .put("/object/test_bucket/test_prefix/test_filename.txt")
            .bytes(image)
            .await;

        put_response.assert_status_ok();
        // todo: assert returned metadata

        let read_response = test_server
            .get("/object/test_bucket/test_prefix/test_filename.txt")
            .await;

        read_response.assert_status_ok();
    }

    #[rstest]
    #[tokio::test]
    async fn test_segment_put_get(test_server: TestServer, text: Bytes) {
        let url = "/object/test_bucket/test_prefix/test_text.txt";
        let put_response = test_server.put(url).bytes(text).await;

        put_response.assert_status_ok();

        let get_response = test_server.get(url).await;
        // todo: assert returned metadata

        get_response.assert_status_ok();
    }

    #[rstest]
    #[tokio::test]
    async fn test_standalone_put_list(test_server: TestServer, image: Bytes) {
        let url = "/object/test_bucket/test_prefix/test_text.txt";
        test_server.put(url).bytes(image.clone()).await;

        let response = test_server.get("/object/test_bucket").await;

        response.assert_status_ok();
        let list_response = response.json::<Vec<Metadata>>();
        assert_eq!(list_response.len(), 1);
        for r in list_response {
            r.test_assert(Metadata::new("test_bucket", "test_prefix", "test_text.txt"));
        }

        test_server.put(url).bytes(image.clone()).await;
        let second_response = test_server.get("/object/test_bucket").await;
        let second_list_response = second_response.json::<Vec<Metadata>>();
        assert_eq!(second_list_response.len(), 2);
        for r in second_list_response {
            r.test_assert(Metadata::new("test_bucket", "test_prefix", "test_text.txt"));
        }
    }
}
