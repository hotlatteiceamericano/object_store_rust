use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// StorageType enum stores the types of storage,
/// including Packed or Standalone.
/// Packed is for those smaller file which will be packed together with other smaller object
/// Standalone is for those bigger file which will use the entire file to store the object
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StoreType {
    Packed {
        segment_file_path: PathBuf,
        offset: u64,
    },
    Standalone {
        file_path: PathBuf,
    },
}
