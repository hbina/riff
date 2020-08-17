use std::fmt::Debug;
use std::io::{Read, Seek};
use std::rc::Rc;

use crate::{
    constants::{LIST_ID, RIFF_ID, SEQT_ID},
    lazy::chunk_id::ChunkIdDisk,
};

#[derive(PartialEq, Debug)]
pub enum ChunkContents<R>
where
    R: Read + Seek,
{
    RawData(ChunkIdDisk<R>, Vec<u8>),
    Children(ChunkIdDisk<R>, ChunkIdDisk<R>, Vec<ChunkContents<R>>),
    ChildrenNoType(ChunkIdDisk<R>, Vec<ChunkContents<R>>),
}

impl<R> std::convert::TryFrom<Chunk<R>> for ChunkContents<R>
where
    R: Read + Seek,
{
    type Error = std::io::Error;

    fn try_from(mut chunk: Chunk<R>) -> Result<Self, std::io::Error> {
        let mut chunk_id = chunk.id()?;
        match chunk_id.as_string()?.as_str() {
            RIFF_ID | LIST_ID => {
                let child_id = chunk.get_child_id()?;
                let child_contents = chunk
                    .iter_type()
                    .map(|child| ChunkContents::try_from(child?))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ChunkContents::Children(chunk_id, child_id, child_contents))
            }
            SEQT_ID => {
                let child_contents = chunk
                    .iter_type()
                    .map(|child| ChunkContents::try_from(child?))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ChunkContents::ChildrenNoType(chunk_id, child_contents))
            }
            _ => {
                let contents = chunk.get_raw_child_content_untyped()?;
                Ok(ChunkContents::RawData(chunk_id, contents))
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Chunk<R>
where
    R: Read + Seek,
{
    pos: u32,
    payload_len: u32,
    reader: Rc<R>,
}

impl<R> Chunk<R>
where
    R: Read + Seek,
{
    pub fn id(&mut self) -> std::io::Result<ChunkIdDisk<R>> {
        Ok(ChunkIdDisk::new(self.reader.clone(), self.pos))
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

    pub fn from_reader(mut reader: Rc<R>, pos: u32) -> std::io::Result<Chunk<R>> {
        let pos = pos as u64;
        let mut payload_buff: [u8; 4] = [0; 4];
        let inner_reader = Rc::get_mut(&mut reader).unwrap();
        inner_reader.seek(std::io::SeekFrom::Start(pos + 4))?;
        inner_reader.read_exact(&mut payload_buff)?;
        Ok(Chunk {
            pos: pos as u32,
            payload_len: u32::from_le_bytes(payload_buff),
            reader,
        })
    }

    pub fn get_child_id(&mut self) -> std::io::Result<ChunkIdDisk<R>> {
        Ok(ChunkIdDisk::new(self.reader.clone(), self.pos + 8))
    }

    pub fn get_child_chunk_typed(&mut self) -> std::io::Result<Chunk<R>> {
        Ok(Chunk::from_reader(self.reader.clone(), self.pos + 12)?)
    }

    pub fn get_child_chunk_untyped(&mut self) -> std::io::Result<Chunk<R>> {
        Ok(Chunk::from_reader(self.reader.clone(), self.pos + 8)?)
    }

    pub fn get_raw_child_content_typed(&mut self) -> std::io::Result<Vec<u8>> {
        let pos = self.pos as u64;
        let payload_len = self.payload_len as usize;
        let reader = Rc::get_mut(&mut self.reader).unwrap();
        reader.seek(std::io::SeekFrom::Start(pos + 12))?;
        let mut result = vec![0; payload_len];
        reader.read_exact(&mut result)?;
        Ok(result)
    }

    pub fn get_raw_child_content_untyped(&mut self) -> std::io::Result<Vec<u8>> {
        let pos = self.pos as u64;
        let payload_len = self.payload_len as usize;
        let reader = Rc::get_mut(&mut self.reader).unwrap();
        reader.seek(std::io::SeekFrom::Start(pos + 8))?;
        let mut result = vec![0; payload_len];
        reader.read_exact(&mut result)?;
        Ok(result)
    }

    pub fn iter_type(&self) -> ChunkIterType<R> {
        ChunkIterType {
            payload_cursor: self.pos + 12,
            payload_end: self.pos + 12 + self.payload_len,
            reader: self.reader.clone(),
        }
    }

    pub fn iter_notype(&self) -> ChunkIterNoType<R> {
        ChunkIterNoType {
            payload_cursor: self.pos + 8,
            payload_end: self.pos + 8 + self.payload_len,
            reader: self.reader.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ChunkIterType<R>
where
    R: Read + Seek,
{
    payload_cursor: u32,
    payload_end: u32,
    reader: Rc<R>,
}

impl<R> Iterator for ChunkIterType<R>
where
    R: Read + Seek,
{
    type Item = std::io::Result<Chunk<R>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.payload_cursor >= self.payload_end {
            None
        } else {
            let chunk = Chunk::from_reader(self.reader.clone(), self.payload_cursor);
            match chunk {
                Ok(chunk) => {
                    self.payload_cursor = self.payload_cursor
                        + 4
                        + 4
                        + 4
                        + chunk.payload_len
                        + (chunk.payload_len % 2);
                    Some(Ok(chunk))
                }
                Err(err) => Some(Err(err)),
            }
        }
    }
}

#[derive(Debug)]
pub struct ChunkIterNoType<R>
where
    R: Read + Seek,
{
    payload_cursor: u32,
    payload_end: u32,
    reader: Rc<R>,
}

impl<R> Iterator for ChunkIterNoType<R>
where
    R: Read + Seek,
{
    type Item = std::io::Result<Chunk<R>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.payload_cursor >= self.payload_end {
            None
        } else {
            let chunk = Chunk::from_reader(self.reader.clone(), self.payload_cursor);
            match chunk {
                Ok(chunk) => {
                    self.payload_cursor = self.payload_cursor
                        + 4
                        + 4
                        + 4
                        + chunk.payload_len
                        + (chunk.payload_len % 2);
                    Some(Ok(chunk))
                }
                Err(err) => Some(Err(err)),
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Riff<R>
where
    R: Read + Seek,
{
    reader: Rc<R>,
}

#[allow(dead_code)]
impl<R> Riff<R>
where
    R: Read + Seek,
{
    pub fn get_chunk(&mut self) -> std::io::Result<Chunk<R>> {
        Chunk::from_reader(self.reader.clone(), 0)
    }

    pub fn from_file(reader: R) -> Self {
        Riff {
            reader: std::rc::Rc::new(reader),
        }
    }
}
