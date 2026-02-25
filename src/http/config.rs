use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub store_path: PathBuf,
}

impl Config {
    pub fn new(store_path: String) -> Self {
        Self {
            store_path: PathBuf::from(store_path),
        }
    }

    pub fn segment_path(&self) -> PathBuf {
        self.store_path.join("segment")
    }

    pub fn standalone_path(&self) -> PathBuf {
        self.store_path.join("standalone")
    }
}
