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
    // how can I prevent this bug?
    // Segment::write calculate the length of serializeed instance
    // may be different from the implementation here
    // one thought:
    // Storable to have a serialize method and ask concrete struct to implement
    // then content_length calls self.serialize().len()
    fn content_length(&self) -> u32 {
        bincode::serialize(&self).unwrap().len() as u32
    }

    fn total_length(&self) -> u32 {
        self.content_length() + 4
    }
}
