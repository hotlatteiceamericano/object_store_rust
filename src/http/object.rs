use std::path::Path;

use axum::{body::Bytes, extract::Path as AxumPath};

pub async fn put_object(AxumPath((bucket, key)): AxumPath<(String, String)>, body: Bytes) {
    let std_path = Path::new(&key);

    let prefixes = std_path.parent().and_then(|p| p.to_str()).unwrap_or("");

    let file_name = std_path
        .file_name()
        .and_then(|p| p.to_str())
        .unwrap_or(&key);

    println!(
        "received PUT request, bucket: {} prefixe: {}, file name: {}, object size: {}",
        bucket,
        prefixes,
        file_name,
        body.len()
    );
}
