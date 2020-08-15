use crate::chunk_id::{ChunkId, LIST_ID, RIFF_ID, SEQT_ID};

#[derive(PartialEq, Debug)]
pub enum ChunkContents<'a> {
    RawData(ChunkId, &'a [u8]),
    Children(ChunkId, ChunkId, Vec<ChunkContents<'a>>),
    ChildrenNoType(ChunkId, Vec<ChunkContents<'a>>),
}

impl<'a> From<Chunk<'a>> for ChunkContents<'a> {
    fn from(chunk: Chunk<'a>) -> Self {
        match chunk.id().as_str() {
            RIFF_ID | LIST_ID => {
                let child_id = chunk.get_child_id();
                let child_contents: Vec<ChunkContents<'a>> = chunk
                    .iter_type()
                    .map(|child| ChunkContents::from(child))
                    .collect();
                ChunkContents::Children(chunk.id(), child_id, child_contents)
            }
            SEQT_ID => {
                let child_contents = chunk
                    .iter_notype()
                    .map(|child| ChunkContents::from(child))
                    .collect();
                ChunkContents::ChildrenNoType(chunk.id(), child_contents)
            }
            _ => {
                let contents = chunk.get_raw_child_content_untyped();
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

    pub fn len(&self) -> u32 {
        self.payload_len
    }

    pub fn is_empty(&self) -> bool {
        self.payload_len == 0
    }

    pub fn offset(&self) -> u32 {
        self.pos
    }

    pub fn from_raw_u8(data: &[u8], pos: u32) -> Chunk {
        let pos = pos as usize;
        let mut buff: [u8; 4] = [0; 4];
        buff.copy_from_slice(&data[pos + 4..pos + 8]);
        Chunk {
            pos: pos as u32,
            payload_len: u32::from_le_bytes(buff),
            data: data,
        }
    }

    pub fn get_child_id(&self) -> ChunkId {
        let pos = self.pos as usize;
        let mut buff: [u8; 4] = [0; 4];
        buff.copy_from_slice(&self.data[pos + 8..pos + 12]);
        ChunkId { value: buff }
    }

    pub fn get_child_chunk_typed(&self) -> Chunk<'a> {
        Chunk::from_raw_u8(self.data, self.pos + 12)
    }

    pub fn get_child_chunk_untyped(&self) -> Chunk<'a> {
        Chunk::from_raw_u8(self.data, self.pos + 8)
    }

    pub fn get_raw_child_content_typed(&self) -> &'a [u8] {
        let pos = self.pos as usize;
        let payload_len = self.payload_len as usize;
        &self.data[pos + 12..pos + 12 + payload_len]
    }

    pub fn get_raw_child_content_untyped(&self) -> &'a [u8] {
        let pos = self.pos as usize;
        let payload_len = self.payload_len as usize;
        &self.data[pos + 8..pos + 8 + payload_len]
    }

    pub fn iter_type(&self) -> ChunkIterType<'a> {
        ChunkIterType {
            cursor: self.pos + 12,
            end: self.data.len() as u32,
            data: self.data,
        }
    }

    pub fn iter_notype(&self) -> ChunkIterNoType<'a> {
        ChunkIterNoType {
            cursor: self.pos + 8,
            end: self.data.len() as u32,
            data: self.data,
        }
    }
}

#[derive(Debug)]
pub struct ChunkIterType<'a> {
    cursor: u32,
    end: u32,
    data: &'a [u8],
}

impl<'a> Iterator for ChunkIterType<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.end {
            None
        } else {
            let chunk = Chunk::from_raw_u8(self.data, self.cursor);
            self.cursor = self.cursor + 4 + 4 + 4 + chunk.payload_len + (chunk.payload_len % 2);
            Some(chunk)
        }
    }
}

#[derive(Debug)]
pub struct ChunkIterNoType<'a> {
    cursor: u32,
    end: u32,
    data: &'a [u8],
}

impl<'a> Iterator for ChunkIterNoType<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.end {
            None
        } else {
            let chunk = Chunk::from_raw_u8(self.data, self.cursor);
            self.cursor = self.cursor + 4 + 4 + chunk.payload_len + (chunk.payload_len % 2);
            Some(chunk)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Riff {
    data: Vec<u8>,
}

#[allow(dead_code)]
impl Riff {
    pub fn len(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn get_chunk(&self) -> Chunk {
        Chunk::from_raw_u8(self.data.as_slice(), 0)
    }

    pub fn from_file(path: std::path::PathBuf) -> std::io::Result<Self> {
        let data = std::fs::read(path)?;
        Ok(Riff { data })
    }
}
