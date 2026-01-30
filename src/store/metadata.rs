use uuid::Uuid;

use crate::common::store_type::{self, StoreType};

// todo: do I need a segment.rs or file.rs to save the object to disk?
pub struct Metadata {
    object_id: Uuid,
    bucket: String,
    name: String,
    prefix: String,

    store_type: Option<StoreType>,
}

impl Metadata {
    pub fn new(bucket: &str, prefix: &str, name: &str) -> Self {
        Self {
            object_id: Uuid::new_v4(),
            bucket: String::from(bucket),
            prefix: String::from(prefix),
            name: String::from(name),
            store_type: None,
        }
    }

    pub fn with_store_type(mut self, store_type: StoreType) -> Self {
        self.store_type = Some(store_type);
        self
    }

    pub fn read(&self, bucket: &str, prefix: &str, name: &str) -> anyhow::Result<()> {
        // todo: determine what type should be returned
        // maynot be the ObjectStore since it should be the actual object being returned
        Ok(())
    }
}
