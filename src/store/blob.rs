use segment_rust::storable::Storable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Blob {
    pub binary: Vec<u8>,
}

impl Blob {
    pub fn new(binary: Vec<u8>) -> Blob {
        Self { binary }
    }
}

impl Storable for Blob {
    fn content_length(&self) -> u32 {
        todo!()
    }

    fn total_length(&self) -> u32 {
        todo!()
    }
}
