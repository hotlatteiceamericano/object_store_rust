use anyhow::Context;
use serde::{Deserialize, Serialize};
use sled::Db;

use crate::common::store_type::StoreType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Metadata {
    // object_id: Uuid,
    pub bucket: String,
    pub filename: String,
    pub prefix: String,
    pub store_type: Option<StoreType>,
}

impl Metadata {
    pub fn new(bucket: &str, prefix: &str, filename: &str) -> Self {
        Self {
            // object_id: Uuid::new_v4(),
            bucket: String::from(bucket),
            prefix: String::from(prefix),
            filename: String::from(filename),
            store_type: None,
        }
    }

    pub fn store_type(&self) -> &Option<StoreType> {
        &self.store_type
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn with_store_type(mut self, store_type: StoreType) -> Self {
        self.store_type = Some(store_type);
        self
    }

    pub fn save(&self, db: &Db) -> anyhow::Result<()> {
        let key = Self::key(&self.bucket, Some(&self.prefix), Some(&self.filename));
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
        let key = Self::key(bucket, Some(prefix), Some(filename));
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

    pub fn list(
        db: &Db,
        bucket: &str,
        prefix: Option<&str>,
        filename: Option<&str>,
    ) -> anyhow::Result<Vec<Metadata>> {
        let mut key = bucket.to_string();
        if prefix.is_some() {
            key = format!("{}/{}", key, &prefix.unwrap());
        }
        if filename.is_some() {
            key = format!("{}/{}", key, filename.unwrap());
        }

        let mut metadata = Vec::new();
        for result in db.scan_prefix(key) {
            let binary = result
                .context("cannot read the value of this sled entry")?
                .1;
            let instance = bincode::deserialize::<Metadata>(&binary)
                .context("cannot deserialize binary to Metadata instance")?;
            metadata.push(instance);
        }

        Ok(metadata)
    }

    fn key(bucket: &str, prefix: Option<&str>, filename: Option<&str>) -> String {
        let mut key = bucket.to_string();
        if prefix.is_some() {
            key = format!("{}/{}", key, &prefix.unwrap());
        }
        if filename.is_some() {
            key = format!("{}/{}", key, filename.unwrap());
        }
        key
    }

    #[cfg(test)]
    pub fn test_assert(&self, other: Metadata) -> bool {
        self.bucket == other.bucket
            && self.filename == other.filename
            && self.prefix == other.prefix
    }
}

#[cfg(test)]
mod test {
    use rstest::{fixture, rstest};
    use sled::Db;

    use crate::store::metadata::Metadata;

    #[fixture]
    fn db() -> Db {
        sled::Config::new()
            .temporary(true)
            .open()
            .expect("cannot open a test sled db")
    }

    #[rstest]
    fn test_save_read(db: Db) {
        let bucket = "test_bucket";
        let prefix = "test_prefix";
        let filename = "test_filename";
        let metadata = Metadata::new(bucket, prefix, filename);

        metadata
            .save(&db)
            .expect("not able to save the metadata during the test");

        let metadata_read = Metadata::read(&db, bucket, prefix, filename)
            .expect("not able to read the metadata from test db")
            .ok_or_else(|| panic!("no metadata in test db"))
            .unwrap();

        assert_eq!(metadata, metadata_read);
    }

    #[rstest]
    fn test_list(db: Db) {
        let first_metadata = Metadata::new("first_bucket", "first_prefix", "first_filename");
        let second_metadata = Metadata::new("second_bucket", "second_prefix", "second_filename");
        let third_metadata = Metadata::new("second_bucket", "third_prefix", "third_filename");

        first_metadata
            .save(&db)
            .expect("not able to save the first metadata in the unit test");
        second_metadata
            .save(&db)
            .expect("not able to save the second metadata in the unit test");
        third_metadata
            .save(&db)
            .expect("not able to save the second metadata in the unit test");

        let list_of_first_metadata = Metadata::list(&db, "first_bucket", None, None)
            .expect("not able to read the metadata from test db");
        assert_eq!(1, list_of_first_metadata.len());

        let list_of_second_metadata = Metadata::list(&db, "second_bucket", None, None)
            .expect("not able to list the metadata from test db");
        assert_eq!(2, list_of_second_metadata.len());
    }
}
