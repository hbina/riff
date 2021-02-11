use crate::{
    constants::{LIST_ID, RIFF_ID, SEQT_ID},
    error::RiffResult,
    FourCC, RiffError,
};
use memmap::Mmap;
use std::convert::TryInto;
use std::path::Path;
use std::{fmt::Debug, fs::File};

#[derive(Debug)]
pub struct Riff {
    inner: Mmap,
}

impl Riff {
    pub fn from_path<P>(path: P) -> RiffResult<Riff>
    where
        P: AsRef<Path>,
    {
        let inner = unsafe { Mmap::map(&File::open(&path)?)? };
        Ok(Riff { inner })
    }

    pub fn as_chunk(&self) -> RiffResult<Chunk> {
        Chunk::from_bytes(&self.inner)
    }
}

/// Represents the possible data contained in a `ChunkDisk`.
#[derive(Debug)]
pub enum Chunk<'a> {
    Raw(&'a [u8]),
    List(&'a [u8]),
    Seqt(&'a [u8]),
}

impl<'a> Chunk<'a> {
    pub fn from_bytes(data: &[u8]) -> RiffResult<Chunk> {
        let id = data.get(0..4).ok_or(RiffError::InsufficientBytes)?;
        let payload_len = u32::from_le_bytes(
            data.get(4..8)
                .ok_or(RiffError::InsufficientBytes)?
                .try_into()?,
        ) as usize;
        let data = data
            .get(0..payload_len + payload_len % 2 + 8)
            .ok_or(RiffError::InsufficientBytes)?;
        match id {
            LIST_ID | RIFF_ID => Ok(Chunk::List(data)),
            SEQT_ID => Ok(Chunk::Seqt(data)),
            _ => Ok(Chunk::Raw(data)),
        }
    }

    pub fn id(&self) -> RiffResult<FourCC> {
        let id = &self.as_bytes()[0..4];
        let result = FourCC::new(id)?;
        Ok(result)
    }

    pub fn payload_len(&self) -> RiffResult<u32> {
        let id = &self.as_bytes()[4..8].try_into().unwrap();
        let result = u32::from_le_bytes(*id);
        Ok(result)
    }

    pub fn chunk_type(&self) -> RiffResult<FourCC> {
        let id = self.read_n_bytes_from_offset(8, 4)?;
        let result = FourCC::new(id)?;
        Ok(result)
    }

    fn read_n_bytes_from_offset(&self, offset: u32, count: u32) -> RiffResult<&[u8]> {
        let pos_begin = offset as usize;
        let pos_end = (offset + count) as usize;
        let data = self
            .as_bytes()
            .get(pos_begin..pos_end)
            .ok_or(RiffError::InsufficientBytes)?;
        Ok(data)
    }

    pub fn content(&self) -> RiffResult<&[u8]> {
        let len = self.payload_len()?;
        self.read_n_bytes_from_offset(0, len)
    }

    pub fn content_offset(&self) -> u32 {
        match self {
            Chunk::Raw(_) => 8,
            Chunk::List(_) => 12,
            Chunk::Seqt(_) => 8,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Chunk::Raw(data) => (data),
            Chunk::List(data) => (data),
            Chunk::Seqt(data) => (data),
        }
    }

    pub fn iter(&self) -> RiffResult<ChunkIter> {
        let offset = self.content_offset();
        let cursor_end = self.as_bytes().len() as u32;
        match self {
            Chunk::Raw(_) => Ok(ChunkIter {
                cursor: 0,
                cursor_end,
                data: self.as_bytes(),
                error_occurred: false,
            }),
            _ => Ok(ChunkIter {
                cursor: offset,
                cursor_end: cursor_end,
                data: self.as_bytes(),
                error_occurred: false,
            }),
        }
    }
}

#[derive(Debug)]
pub struct ChunkIter<'a> {
    cursor: u32,
    cursor_end: u32,
    data: &'a [u8],
    error_occurred: bool,
}

macro_rules! try_result {
    ( $self : ident , $value :expr ) => {
        match $value {
            Ok(r) => r,
            Err(err) => {
                $self.error_occurred = true;
                return Some(Err(err));
            }
        }
    };
}

macro_rules! try_option {
    ( $self : ident , $value :expr ) => {
        match $value {
            Some(r) => r,
            None => return Some(Err(RiffError::InsufficientBytes)),
        }
    };
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = RiffResult<Chunk<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error_occurred || self.cursor >= self.cursor_end {
            None
        } else {
            let cursor = self.cursor as usize;
            let cursor_end = self.cursor_end as usize;
            let data = try_option!(self, self.data.get(cursor..cursor_end));
            let chunk = try_result!(self, Chunk::from_bytes(data));
            let payload_len = try_result!(self, chunk.payload_len());
            self.cursor += payload_len + payload_len % 2;
            Some(Ok(chunk))
        }
    }
}
