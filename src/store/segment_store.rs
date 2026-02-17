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
    const MAX_LENGTH: u64 = 1 * 1024;
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

        let biggest_segment: Option<Segment<Blob>> = fs::read_dir(&path)
            .context("cannot read directory")?
            .filter_map(|r| r.ok())
            .map(|entry| entry.path())
            .filter(|p| p.extension().unwrap() == segment::FILE_EXTENSION)
            .map(|p| Segment::from(&p))
            .max_by(|a, b| a.base_offset().cmp(&b.base_offset()));

        match biggest_segment {
            Some(b) if b.write_position() < Self::MAX_LENGTH => Ok(b),
            Some(b) => Segment::new(&path, b.write_position())
                .context("cannot create a new bigger segment"),
            None => Segment::new(&path, 0),
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

    fn rotate_segment(&mut self, new_base_offset: u64) -> anyhow::Result<()> {
        self.active_segment = Segment::new(&Self::path(), new_base_offset)
            .context("failed to create a new segment during rotation")?;
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
            println!("rotating to a new segment");
            self.rotate_segment(
                self.active_segment.base_offset() + self.active_segment.write_position(),
            )
            .context("failed to rotate to a new segment")?;
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
    use rstest::{fixture, rstest};

    use crate::{
        common::store_type::StoreType,
        store::{object_store::ObjectStore, segment_store::SegmentStore},
    };

    #[fixture]
    fn segment_store() -> SegmentStore {
        SegmentStore::new().unwrap()
    }

    #[fixture]
    fn text_binary() -> Vec<u8> {
        fs::read(
            std::env::current_dir()
                .unwrap()
                .join("test")
                .join("test.txt"),
        )
        .unwrap()
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_open(mut segment_store: SegmentStore, text_binary: Vec<u8>) {
        let store_type = segment_store
            .save(&Bytes::from(text_binary.clone()))
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
            binary_read, text_binary,
            "binary read from the segment store is different than the original binary",
        );
    }

    #[rstest]
    #[tokio::test]
    async fn test_segment_rotation_multiple_times() {
        todo!(
            "test the rotation by save() enough times using the test.txt for it to rotate two times"
        );
    }
}
