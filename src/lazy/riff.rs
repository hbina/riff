use std::convert::TryFrom;
use std::fmt::Debug;
use std::io::{Read, Seek};
use std::rc::Rc;

use crate::constants::{LIST_ID, RIFF_ID, SEQT_ID};

/// Lazy version of `ChunkId`.
#[derive(Debug, Clone)]
pub struct ChunkId {
    data: [u8; 4],
}

impl ChunkId {
    pub fn as_str(&self) -> &str {
        // TODO: Handle this error.
        std::str::from_utf8(&self.data).unwrap()
    }
}

/// Lazy version of `ChunkType`.
#[derive(Debug, Clone)]
pub struct ChunkType {
    data: [u8; 4],
}

impl ChunkType {
    pub fn as_str(&self) -> &str {
        // TODO: Handle this error.
        std::str::from_utf8(&self.data).unwrap()
    }
}

#[derive(Debug)]
pub enum ChunkContents {
    RawData(ChunkId, Vec<u8>),
    Children(ChunkId, ChunkType, Vec<ChunkContents>),
    ChildrenNoType(ChunkId, Vec<ChunkContents>),
}

impl<'a, R> std::convert::TryFrom<Chunk<R>> for ChunkContents
where
    R: Read + Seek,
{
    type Error = std::io::Error;

    fn try_from(chunk: Chunk<R>) -> Result<Self, std::io::Error> {
        let chunk_id = chunk.id().clone();
        match chunk_id.as_str() {
            RIFF_ID | LIST_ID => {
                let chunk_type = chunk.chunk_type();
                let child_contents = chunk
                    .iter()
                    .map(|child| ChunkContents::try_from(child))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ChunkContents::Children(
                    chunk_id,
                    chunk_type.clone(),
                    child_contents,
                ))
            }
            SEQT_ID => {
                let child_contents = chunk
                    .iter()
                    .map(|child| ChunkContents::try_from(child))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ChunkContents::ChildrenNoType(chunk_id, child_contents))
            }
            _ => {
                let contents = chunk.get_raw_child()?;
                Ok(ChunkContents::RawData(chunk_id.clone(), contents))
            }
        }
    }
}

#[derive(Debug)]
pub struct Chunk<R>
where
    R: Read + Seek,
{
    id: ChunkId,
    chunk_type: ChunkType,
    pos: u32,
    payload_len: u32,
    reader: Rc<R>,
}

impl<R> Chunk<R>
where
    R: Read + Seek,
{
    pub fn id(&self) -> &ChunkId {
        &self.id
    }

    pub fn payload_len(&self) -> u32 {
        self.payload_len
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn from_reader(mut reader: Rc<R>, pos: u32) -> std::io::Result<Chunk<R>> {
        let pos = pos as u64;
        let inner_reader = Rc::get_mut(&mut reader).unwrap();
        let id_buff = Chunk::read_4_bytes(inner_reader, 0)?;
        let payload_len_buff = Chunk::read_4_bytes(inner_reader, 4)?;
        let chunk_type_buff = Chunk::read_4_bytes(inner_reader, 8)?;
        Ok(Chunk {
            id: ChunkId { data: id_buff },
            chunk_type: ChunkType {
                data: chunk_type_buff,
            },
            pos: pos as u32,
            payload_len: u32::from_le_bytes(payload_len_buff),
            reader: reader.clone(),
        })
    }

    fn read_4_bytes(reader: &mut R, pos: u64) -> std::io::Result<[u8; 4]> {
        let mut buffer: [u8; 4] = [0; 4];
        reader.seek(std::io::SeekFrom::Start(pos))?;
        reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn get_raw_child(&self) -> std::io::Result<Vec<u8>> {
        let pos = self.pos as u64;
        let payload_len = self.payload_len as usize;
        let offset = self.offset_into_data() as u64;
        let mut result = vec![0; payload_len];
        let mut rr = self.reader.clone();
        let reader = Rc::get_mut(&mut rr).unwrap();
        reader.seek(std::io::SeekFrom::Start(pos + offset))?;
        reader.read_exact(&mut result)?;
        Ok(result)
    }

    fn offset_into_data(&self) -> usize {
        match self.id().as_str() {
            RIFF_ID | LIST_ID => 12,
            _ => 8,
        }
    }

    pub fn iter(&self) -> ChunkIter<R> {
        let offset = self.offset_into_data() as u32;
        ChunkIter {
            cursor: self.pos + offset,
            cursor_end: self.pos + offset + self.payload_len,
            reader: self.reader.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ChunkIter<R>
where
    R: Read + Seek,
{
    cursor: u32,
    cursor_end: u32,
    reader: Rc<R>,
}

impl<R> Iterator for ChunkIter<R>
where
    R: Read + Seek,
{
    type Item = Chunk<R>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.cursor_end {
            None
        } else {
            let chunk = Chunk::from_reader(self.reader.clone(), self.cursor).unwrap();
            self.cursor = self.cursor + 8 + chunk.payload_len + (chunk.payload_len % 2);
            Some(chunk)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Riff<R>
where
    R: Read + Seek,
{
    reader: Rc<R>,
}

impl<R> TryFrom<Riff<R>> for Chunk<R>
where
    R: Seek + Read,
{
    type Error = std::io::Error;

    fn try_from(value: Riff<R>) -> Result<Self, Self::Error> {
        Chunk::from_reader(value.reader, 0)
    }
}

#[allow(dead_code)]
impl<R> Riff<R>
where
    R: Read + Seek,
{
    pub fn from_file(reader: R) -> std::io::Result<Self> {
        Ok(Riff {
            reader: Rc::new(reader),
        })
    }
}
