use std::{fs, path::PathBuf};

use anyhow::{Context, Ok, Result};
use axum::body::Bytes;
use segment_rust::segment::{self, Segment};

use crate::{
    common::store_type::StoreType,
    store::{blob::Blob, object_store::ObjectStore},
};

pub struct SegmentStore {
    active_segment: Segment<Blob>,
}

impl SegmentStore {
    const MAX_LENGTH: u64 = 10 * 1024 * 1042;
    pub fn new() -> Result<Self> {
        let active_segment = Self::find_active_segment().context("cannot find active segment")?;
        Ok(Self { active_segment })
    }

    /// Find the segment under data/segment/ which is not full yet
    /// If there are no segment found, will create a new one
    fn find_active_segment() -> anyhow::Result<Segment<Blob>> {
        let path = Self::path();

        let segment: Option<Segment<Blob>> = fs::read_dir(&path)
            .context("cannot read directory")?
            .filter_map(|r| r.ok())
            .map(|entry| entry.path())
            .filter(|p| p.extension().unwrap() == segment::FILE_EXTENSION)
            .map(|p| Segment::from(p))
            .find(|s| s.write_position() < Self::MAX_LENGTH);

        match segment {
            Some(s) => return Ok(s),
            None => return Segment::new(&path, 0),
        }
    }

    fn rotate_segment(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}

impl ObjectStore for SegmentStore {
    async fn save(
        &mut self,
        bytes: &Bytes,
    ) -> anyhow::Result<crate::common::store_type::StoreType> {
        let blob = Blob::new(bytes.to_vec());
        self.active_segment
            .write(&blob)
            .context("failed to write blob using SegmentStore")?;

        if self.active_segment.write_position() > Self::MAX_LENGTH {
            self.rotate_segment()?;
        }

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

    async fn open(
        // todo: argument to take segment file path, offset and length
        file_name: &str,
    ) -> anyhow::Result<futures::stream::BoxStream<'static, Result<Bytes, futures::io::Error>>>
    {
        todo!()
    }
}
