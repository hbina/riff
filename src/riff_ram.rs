use crate::chunk_id::{ChunkId, ChunkType, LIST_ID, RIFF_ID, SEQT_ID};

#[derive(Debug)]
pub enum ChunkContents<'a> {
    RawData(ChunkId, &'a [u8]),
    Children(ChunkId, ChunkType, Vec<ChunkContents<'a>>),
    ChildrenNoType(ChunkId, Vec<ChunkContents<'a>>),
}

impl<'a> From<Chunk<'a>> for ChunkContents<'a> {
    fn from(chunk: Chunk<'a>) -> Self {
        match chunk.id().as_str() {
            RIFF_ID | LIST_ID => {
                let chunk_type = chunk.get_chunk_type();
                let child_contents: Vec<ChunkContents<'a>> = chunk
                    .iter()
                    .map(|child| ChunkContents::from(child))
                    .collect();
                ChunkContents::Children(chunk.id(), chunk_type, child_contents)
            }
            SEQT_ID => {
                let child_contents = chunk
                    .iter()
                    .map(|child| ChunkContents::from(child))
                    .collect();
                ChunkContents::ChildrenNoType(chunk.id(), child_contents)
            }
            _ => {
                let contents = chunk.get_raw_child();
                ChunkContents::RawData(chunk.id(), contents)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk<'a> {
    pos: u32,
    payload_len: u32,
    data: &'a [u8],
}

impl<'a> Chunk<'a> {
    pub fn id(&self) -> ChunkId {
        let pos = self.pos as usize;
        let mut buff: [u8; 4] = [0; 4];
        buff.copy_from_slice(&self.data[pos..pos + 4]);
        ChunkId { value: buff }
    }

    pub fn from_raw_u8(data: &[u8], pos: u32) -> Chunk {
        let pos = pos as usize;
        let mut payload_buff: [u8; 4] = [0; 4];
        payload_buff.copy_from_slice(&data[pos + 4..pos + 8]);
        Chunk {
            pos: pos as u32,
            payload_len: u32::from_le_bytes(payload_buff),
            data,
        }
    }

    pub fn get_chunk_type(&self) -> ChunkType {
        let pos = self.pos as usize;
        let mut buff: [u8; 4] = [0; 4];
        buff.copy_from_slice(&self.data[pos + 8..pos + 12]);
        ChunkType { value: buff }
    }

    pub fn get_raw_child(&self) -> &'a [u8] {
        let pos = self.pos as usize;
        let payload_len = self.payload_len as usize;
        match self.id().as_str() {
            RIFF_ID | LIST_ID => &self.data[pos + 12..pos + 12 + payload_len],
            _ => &self.data[pos + 8..pos + 8 + payload_len],
        }
    }

    pub fn iter(&self) -> ChunkIter<'a> {
        match self.id().as_str() {
            RIFF_ID | LIST_ID => ChunkIter {
                cursor: self.pos + 12,
                // We have to subtract because RIFF_ID and LIST_ID contain chunk type that consumes 4 bytes.
                cursor_end: self.pos + 12 + self.payload_len - 4,
                data: self.data,
            },
            _ => ChunkIter {
                cursor: self.pos + 8,
                cursor_end: self.pos + 8 + self.payload_len,
                data: self.data,
            },
        }
    }
}

#[derive(Debug)]
pub struct ChunkIter<'a> {
    cursor: u32,
    cursor_end: u32,
    data: &'a [u8],
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.cursor_end {
            None
        } else {
            let chunk = Chunk::from_raw_u8(self.data, self.cursor);
            self.cursor = self.cursor + 8 + chunk.payload_len + (chunk.payload_len % 2);
            Some(chunk)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Riff {
    data: Vec<u8>,
}

impl<'a> From<&'a Riff> for Chunk<'a> {
    fn from(value: &'a Riff) -> Self {
        Chunk::from_raw_u8(&value.data, 0)
    }
}

#[allow(dead_code)]
impl Riff {
    pub fn id(&self) -> ChunkId {
        let mut buff: [u8; 4] = [0; 4];
        buff.copy_from_slice(&self.data[0..4]);
        ChunkId { value: buff }
    }

    pub fn payload_len(&self) -> u32 {
        let mut buff: [u8; 4] = [0; 4];
        buff.copy_from_slice(&self.data[4..8]);
        u32::from_le_bytes(buff)
    }

    pub fn iter(&self) -> ChunkIter {
        Chunk::from_raw_u8(self.data.as_slice(), 0).iter()
    }

    pub fn from_file(path: std::path::PathBuf) -> std::io::Result<Self> {
        let data = std::fs::read(path)?;
        Ok(Riff { data })
    }
}
