use std::io::{Read, Seek};
use std::rc::Rc;

#[derive(Debug)]
pub struct ChunkId {
    pub value: [u8; 4],
}

impl ChunkId {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.value).unwrap()
    }
}
