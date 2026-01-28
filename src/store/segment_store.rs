use std::path::{Path, PathBuf};

use anyhow::Ok;
use axum::body::Bytes;

use crate::{common::store_type::StoreType, store::object_store::ObjectStore};

pub struct SegmentStore {}

impl SegmentStore {
    pub fn new() -> Self {
        Self {}
    }
}
impl ObjectStore for SegmentStore {
    fn save(&self, bytes: Bytes) -> anyhow::Result<crate::common::store_type::StoreType> {
        println!("saving the object to a segment!");
        Ok(StoreType::Packed {
            segment_file_path: PathBuf::from("./data/segments/1.segment"),
            offset: 0,
            length: 0,
        })
    }

    fn path() -> PathBuf {
        std::env::current_dir()
            .unwrap()
            .join("store")
            .join("segment")
    }
}
