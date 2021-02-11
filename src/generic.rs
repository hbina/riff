use crate::{
    constants::{LIST_ID, RIFF_ID, SEQT_ID},
    error::RiffResult,
    FourCC,
};
use memmap::Mmap;
use std::convert::TryInto;
use std::path::Path;
use std::{fmt::Debug, fs::File, rc::Rc};

type RcMmap = std::rc::Rc<Mmap>;

/// Represents the possible data contained in a `ChunkDisk`.
#[derive(Debug)]
pub enum ChunkDiskType {
    RawData(Chunk),
    Children(Chunk),
    ChildrenNoType(Chunk),
}

impl ChunkDiskType {
    pub fn from_chunk_disk(chunk: Chunk) -> RiffResult<ChunkDiskType> {
        let chunk_id = chunk.id()?;
        let result = match chunk_id.as_bytes() {
            RIFF_ID | LIST_ID => ChunkDiskType::Children(chunk),
            SEQT_ID => ChunkDiskType::ChildrenNoType(chunk),
            _ => ChunkDiskType::RawData(chunk),
        };
        Ok(result)
    }
}

/// `ChunkDisk` is an opaque type. The only way to access its content is by converting it into
/// a `ChunkDiskContent`.

/// Represents a lazy reader of a chunk in a RIFF file.
#[derive(Debug)]
pub struct Chunk {
    offset: u32,
    reader: RcMmap,
}

impl Chunk {
    pub fn id(&self) -> RiffResult<FourCC> {
        let id = self.read_4_bytes_from_offset(0)?;
        let result = FourCC::new(id)?;
        Ok(result)
    }

    pub fn payload_len(&self) -> RiffResult<u32> {
        let id = self.read_4_bytes_from_offset(4)?;
        let result = u32::from_le_bytes(*id);
        Ok(result)
    }

    pub fn chunk_type(&self) -> RiffResult<FourCC> {
        let id = self.read_4_bytes_from_offset(8)?;
        let result = FourCC::new(id)?;
        Ok(result)
    }

    fn with_mmap_and_offset(mmap: RcMmap, offset: u32) -> Chunk {
        Chunk {
            offset,
            reader: mmap,
        }
    }

    pub fn from_path<P>(path: P) -> RiffResult<Chunk>
    where
        P: AsRef<Path>,
    {
        let reader = unsafe { Rc::new(Mmap::map(&File::open(&path)?)?) };
        Ok(Chunk { offset: 0, reader })
    }

    fn read_4_bytes_from_offset(&self, offset: u32) -> RiffResult<&[u8; 4]> {
        let pos = (self.offset + offset) as usize;
        let reader = &self.reader[pos..pos + 4];
        let arr_ref: &[u8; 4] = reader.try_into()?;
        Ok(arr_ref)
    }

    pub fn get_raw_child(&self) -> RiffResult<&[u8]> {
        let pos = self.offset as usize;
        let payload_len = self.payload_len()? as usize;
        let offset = self.offset_into_data()?;
        let begin_idx = pos + offset;
        let end_idx = begin_idx + payload_len;
        let reader = &self.reader[begin_idx..end_idx];
        Ok(reader)
    }

    fn offset_into_data(&self) -> RiffResult<usize> {
        Ok(match self.id()?.as_bytes() {
            RIFF_ID | LIST_ID => 12,
            _ => 8,
        })
    }

    pub fn iter(&self) -> RiffResult<ChunkDiskIter> {
        let result = match self.id()?.as_bytes() {
            LIST_ID | RIFF_ID => ChunkDiskIter {
                cursor: self.offset + 12,
                cursor_end: self.offset + 12 + self.payload_len()? - 4,
                reader: self.reader.clone(),
                error_occurred: false,
            },
            _ => ChunkDiskIter {
                cursor: self.offset + 8,
                cursor_end: self.offset + 8 + self.payload_len()?,
                reader: self.reader.clone(),
                error_occurred: false,
            },
        };
        Ok(result)
    }
}

#[derive(Debug)]
pub struct ChunkDiskIter {
    cursor: u32,
    cursor_end: u32,
    reader: RcMmap,
    error_occurred: bool,
}

impl Iterator for ChunkDiskIter {
    type Item = RiffResult<Chunk>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error_occurred || self.cursor >= self.cursor_end {
            None
        } else {
            let chunk = Chunk::with_mmap_and_offset(self.reader.clone(), self.cursor);
            let payload = chunk.payload_len();
            match payload {
                Ok(len) => {
                    let chunk_size = 8 + len + (len % 2);
                    self.cursor += chunk_size;
                    Some(Ok(chunk))
                }
                Err(err) => {
                    self.error_occurred = true;
                    Some(Err(err))
                }
            }
        }
    }
}
