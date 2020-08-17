#[derive(Debug)]
pub struct ChunkType {
    pub value: [u8; 4],
}

impl ChunkType {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.value).unwrap()
    }
}
