use uuid::Uuid;

use crate::common::storage_type::{self, StorageType};

// todo: do I need a segment.rs or file.rs to save the object to disk?
pub struct Metadata {
    object_id: Uuid,
    bucket: String,
    name: String,
    prefix: String,

    storage_type: Option<StorageType>,
}

impl Metadata {
    pub fn new(bucket: &str, prefix: &str, name: &str) -> Self {
        Self {
            object_id: Uuid::new_v4(),
            bucket: String::from(bucket),
            prefix: String::from(prefix),
            name: String::from(name),
            storage_type: None,
        }
    }

    pub fn save(&self, binary: &[u8]) -> anyhow::Result<()> {
        // todo: implement the functionality to write the binary to disk
        Ok(())
    }

    pub fn read(&self, bucket: &str, prefix: &str, name: &str) -> anyhow::Result<()> {
        // todo: determine what type should be returned
        // maynot be the ObjectStore since it should be the actual object being returned
        Ok(())
    }
}
