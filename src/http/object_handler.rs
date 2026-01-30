use std::path::Path;

use axum::{
    body::Bytes,
    extract::{Path as AxumPath, State},
    response::IntoResponse,
};
use object_store_rust::{
    common::store_type::StoreType,
    store::{
        app_error::AppError, metadata::Metadata, object_store::ObjectStore,
        standalone_store::StandaloneStore,
    },
};

use crate::http::app_state::AppState;

pub const SMALL_OBJECT_SIZE_THRESHOLD: usize = 30 * 1024 * 1024;

pub async fn put_object(
    State(state): State<AppState>,
    AxumPath((bucket, key)): AxumPath<(String, String)>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let std_path = Path::new(&key);
    let prefix = std_path.parent().and_then(|p| p.to_str()).unwrap_or("");
    let filename = std_path
        .file_name()
        .and_then(|p| p.to_str())
        .unwrap_or(&key);

    println!(
        "received PUT request, bucket: {} prefixe: {}, file name: {}, object size: {}",
        bucket,
        prefix,
        filename,
        body.len()
    );

    let store_type = if body.len() <= SMALL_OBJECT_SIZE_THRESHOLD {
        // let storer = SegmentStore::new();
        let store = StandaloneStore::new();
        store.save(&body)

        // save the metadata using the result from store.save
    } else {
        let store = StandaloneStore::new();
        store.save(&body)
    }?;

    let metadata = Metadata::new(&bucket, &prefix, &filename).with_store_type(store_type);
    metadata.save(&state.db)?;

    Ok(format!("successfully save metadata: {:?}", metadata))
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
