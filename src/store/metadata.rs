use anyhow::Context;
use serde::{Deserialize, Serialize};
use sled::Db;

use crate::common::store_type::StoreType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    // object_id: Uuid,
    bucket: String,
    filename: String,
    prefix: String,
    store_type: Option<StoreType>,
}

impl Metadata {
    pub fn new(bucket: &str, prefix: &str, name: &str) -> Self {
        Self {
            // object_id: Uuid::new_v4(),
            bucket: String::from(bucket),
            prefix: String::from(prefix),
            filename: String::from(name),
            store_type: None,
        }
    }

    pub fn with_store_type(mut self, store_type: StoreType) -> Self {
        self.store_type = Some(store_type);
        self
    }

    pub fn save(&self, db: &Db) -> anyhow::Result<()> {
        let key = self.key();
        let bytes = bincode::serialize(self).context("failed to serialize metadata")?;
        db.insert(key.as_bytes(), bytes)
            .context("failed to insert metadata to db")?;
        Ok(())
    }

    /// # Returns
    /// Serialization and other db failues may occur,
    /// on top of missing keys
    pub fn read(
        db: &Db,
        bucket: &str,
        prefix: &str,
        filename: &str,
    ) -> anyhow::Result<Option<Self>> {
        let key = format!("{}/{}/{}", bucket, prefix, filename);
        let bytes = db
            .get(key.as_bytes())
            .context("cannot find the metadata from db")?;

        bytes
            .map(|b| {
                bincode::deserialize::<Metadata>(&b)
                    .context("failed to deserialize sled binaries into Metadata instance")
            })
            .transpose()
    }

    fn key(&self) -> String {
        format!("{}/{}/{}", self.bucket, self.prefix, self.filename)
    }
}
