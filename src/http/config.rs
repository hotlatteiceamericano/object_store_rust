use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub segment_path: PathBuf,
    pub standalone_path: PathBuf,
}

impl Config {
    pub fn new(segment_path: String, standalone_path: String) -> Self {
        Self {
            segment_path: PathBuf::from(segment_path),
            standalone_path: PathBuf::from(standalone_path),
        }
    }
}
