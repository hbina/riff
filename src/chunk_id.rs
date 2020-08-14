#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ChunkId {
    pub value: [u8; 4],
}

pub const RIFF_ID: ChunkId = ChunkId {
    value: [0x52, 0x49, 0x46, 0x46],
};

pub const LIST_ID: ChunkId = ChunkId {
    value: [0x4C, 0x49, 0x53, 0x54],
};

pub const SEQT_ID: ChunkId = ChunkId {
    value: [0x73, 0x65, 0x71, 0x74],
};

impl ChunkId {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.value).unwrap()
    }

    pub fn new(s: &str) -> Option<ChunkId> {
        let bytes = s.as_bytes();
        if bytes.len() != 4 {
            None
        } else {
            let mut a: [u8; 4] = Default::default();
            a.copy_from_slice(&bytes[..]);
            Some(ChunkId { value: a })
        }
    }
}
