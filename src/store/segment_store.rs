use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use axum::body::Bytes;
use futures::{StreamExt, stream};
use segment_rust::segment::{self, Segment};

use crate::{
    common::store_type::StoreType,
    store::{blob::Blob, object_store::ObjectStore},
};

/// Store object in segments
/// It finds and store an active segment in it as a field
/// store object in it until the active segment is full
/// then rotate to a new segment
pub struct SegmentStore {
    active_segment: Segment<Blob>,
}

impl SegmentStore {
    const MAX_LENGTH: u64 = 10 * 1024 * 1042;
    pub fn new() -> Result<Self> {
        let active_segment = Self::find_active_segment().context("cannot find active segment")?;
        Ok(Self { active_segment })
    }

    /// Find the segment under store/segment/ which is not full yet
    /// If there are no segment found, will create a new one
    fn find_active_segment() -> anyhow::Result<Segment<Blob>> {
        let path = Self::path();
        if !path.exists() {
            fs::create_dir_all(&path).expect("cannot create parent directory for SegmentStore");
        }

        let segment: Option<Segment<Blob>> = fs::read_dir(&path)
            .context("cannot read directory")?
            .filter_map(|r| r.ok())
            .map(|entry| entry.path())
            .filter(|p| p.extension().unwrap() == segment::FILE_EXTENSION)
            .map(|p| Segment::from(&p))
            .find(|s| s.write_position() < Self::MAX_LENGTH);

        match segment {
            Some(s) => return Ok(s),
            None => return Segment::new(&path, 0),
        }
    }

    pub async fn open(
        file_path: &PathBuf,
        offset: u64,
    ) -> anyhow::Result<futures::stream::BoxStream<'static, Result<Bytes, futures::io::Error>>>
    {
        let mut segment = Segment::<Blob>::from(file_path);
        let blob = segment.read(offset)?;
        let stream_of_bytes = stream::once(async { Ok(Bytes::from(blob.binary)) });

        Ok(stream_of_bytes.boxed())
    }

    fn rotate_segment(&mut self) -> anyhow::Result<()> {
        self.active_segment = Segment::new(&Self::path(), 0)?;
        Ok(())
    }
}

impl ObjectStore for SegmentStore {
    async fn save(
        &mut self,
        bytes: &Bytes,
    ) -> anyhow::Result<crate::common::store_type::StoreType> {
        let curr_offset = self.active_segment.write_position();
        let curr_path = self.active_segment.path().to_owned();
        let blob = Blob::new(bytes.to_vec());

        self.active_segment
            .write(&blob)
            .context("failed to write blob using SegmentStore")?;

        if self.active_segment.write_position() > Self::MAX_LENGTH {
            self.rotate_segment()?;
        }

        Ok(StoreType::Packed {
            segment_file_path: curr_path,
            offset: curr_offset,
        })
    }

    fn path() -> PathBuf {
        std::env::current_dir()
            .unwrap()
            .join("store")
            .join("segment")
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use axum::body::Bytes;
    use futures::StreamExt;
    use rstest::rstest;

    use crate::{
        common::store_type::StoreType,
        store::{object_store::ObjectStore, segment_store::SegmentStore},
    };

    #[rstest]
    #[tokio::test]
    async fn test_save_open() {
        let mut segment_store = SegmentStore::new().unwrap();
        let text_file_bytes = fs::read(
            std::env::current_dir()
                .unwrap()
                .join("test")
                .join("test.txt"),
        )
        .unwrap();

        let store_type = segment_store
            .save(&Bytes::from(text_file_bytes.clone()))
            .await
            .unwrap();

        let StoreType::Packed {
            segment_file_path,
            offset,
        } = store_type
        else {
            panic!("SegmentStore returns a standalone store type");
        };

        let mut stream = SegmentStore::open(&segment_file_path, offset)
            .await
            .expect("SegmentStore cannot open stream");
        let mut binary_read = Vec::<u8>::new();
        while let Some(chunk) = stream.next().await {
            binary_read.extend_from_slice(&chunk.unwrap());
        }

        assert_eq!(
            binary_read, text_file_bytes,
            "binary read from the segment store is different",
        );
    }

    fn init() {
        let path = SegmentStore::path();
    }
}
