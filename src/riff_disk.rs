use crate::chunk_id::{ChunkId, LIST_ID, RIFF_ID, SEQT_ID};
use std::io::{Read, Seek};

#[derive(PartialEq, Debug)]
pub enum ChunkContents {
    RawData(ChunkId, Vec<u8>),
    Children(ChunkId, ChunkId, Vec<ChunkContents>),
    ChildrenNoType(ChunkId, Vec<ChunkContents>),
}

impl<'a, R> std::convert::TryFrom<Chunk<'a, R>> for ChunkContents
where
    R: Read + Seek,
{
    type Error = &'static str;

    fn try_from(chunk: Chunk<R>) -> Result<Self, &'static str> {
        match chunk.id() {
            RIFF_ID | LIST_ID => {
                let child_id = chunk.get_child_id();
                let child_contents: Vec<Result<ChunkContents, &'static str>> = chunk
                    .iter_type()
                    .map(|child| ChunkContents::try_from(child))
                    .collect();
                if child_contents.iter().any(|x| x.is_err()) {
                    Err("unable to convert Chunk into ChunkContent")
                } else {
                    Ok(ChunkContents::Children(
                        chunk.id(),
                        child_id,
                        child_contents.iter().map(|x| x.unwrap()).collect(),
                    ))
                }
            }
            SEQT_ID => {
                let child_contents = chunk
                    .iter_type()
                    .map(|child| ChunkContents::try_from(child));
                if child_contents.any(|x| x.is_err()) {
                    Err("unable to convert Chunk into ChunkContent")
                } else {
                    Ok(ChunkContents::ChildrenNoType(
                        chunk.id(),
                        child_contents.map(|x| x.unwrap()).collect(),
                    ))
                }
            }
            _ => match chunk.get_raw_child_content_untyped() {
                Ok(contents) => Ok(ChunkContents::RawData(chunk.id(), contents)),
                Err(err) => Err("hello"),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Chunk<'a, R>
where
    R: Read + Seek,
{
    pos: u32,
    payload_len: u32,
    reader: &'a mut R,
}

impl<'a, R> Chunk<'a, R>
where
    R: Read + Seek,
{
    pub fn id(&mut self) -> ChunkId {
        let pos = self.pos as u64;
        let mut buff: [u8; 4] = [0; 4];
        self.reader.seek(std::io::SeekFrom::Start(pos));
        self.reader.read_exact(&mut buff);
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

    pub fn from_reader(reader: &'_ mut R, pos: u32) -> Chunk<'_, R> {
        let pos = pos as u64;
        let mut payload_buff: [u8; 4] = [0; 4];
        reader.seek(std::io::SeekFrom::Start(pos));
        reader.read_exact(&mut payload_buff);
        Chunk {
            pos: pos as u32,
            payload_len: u32::from_le_bytes(payload_buff),
            reader,
        }
    }

    pub fn get_child_id(&mut self) -> ChunkId {
        let pos = self.pos as u64;
        let mut buff: [u8; 4] = [0; 4];
        self.reader.seek(std::io::SeekFrom::Start(pos + 8));
        self.reader.read_exact(&mut buff);
        ChunkId { value: buff }
    }

    pub fn get_child_chunk_typed(&mut self) -> Chunk<R> {
        Chunk::from_reader(self.reader, self.pos + 12)
    }

    pub fn get_child_chunk_untyped(&mut self) -> Chunk<R> {
        Chunk::from_reader(self.reader, self.pos + 8)
    }

    pub fn get_raw_child_content_typed(&self) -> std::io::Result<Vec<u8>> {
        let pos = self.pos as u64;
        let payload_len = self.payload_len as usize;
        self.reader.seek(std::io::SeekFrom::Start(pos + 12))?;
        let result = vec![0; payload_len];
        self.reader.read_exact(&mut result);
        Ok(result)
    }

    pub fn get_raw_child_content_untyped(&self) -> std::io::Result<Vec<u8>> {
        let pos = self.pos as u64;
        let payload_len = self.payload_len as usize;
        self.reader.seek(std::io::SeekFrom::Start(pos + 8))?;
        let result = vec![0; payload_len];
        self.reader.read_exact(&mut result);
        Ok(result)
    }

    pub fn iter_type(&mut self) -> ChunkIterType<R> {
        ChunkIterType {
            payload_cursor: self.pos + 12,
            payload_end: self.pos + 12 + self.payload_len,
            reader: self.reader,
        }
    }

    pub fn iter_notype(&mut self) -> ChunkIterNoType<R> {
        ChunkIterNoType {
            payload_cursor: self.pos + 8,
            payload_end: self.pos + 8 + self.payload_len,
            reader: self.reader,
        }
    }
}

#[derive(Debug)]
pub struct ChunkIterType<'a, R>
where
    R: Read + Seek,
{
    payload_cursor: u32,
    payload_end: u32,
    reader: &'a mut R,
}

impl<'a, R> Iterator for ChunkIterType<'a, R>
where
    R: Read + Seek,
{
    type Item = Chunk<'a, R>;

    fn next(&mut self) -> Option<'_, Self::Item> {
        if self.payload_cursor >= self.payload_end {
            None
        } else {
            let chunk = Chunk::from_reader(self.reader, self.payload_cursor);
            self.payload_cursor =
                self.payload_cursor + 4 + 4 + 4 + chunk.payload_len + (chunk.payload_len % 2);
            Some(chunk)
        }
    }
}

#[derive(Debug)]
pub struct ChunkIterNoType<'a, R>
where
    R: Read + Seek,
{
    payload_cursor: u32,
    payload_end: u32,
    reader: &'a mut R,
}

impl<'a, R> Iterator for ChunkIterNoType<'a, R>
where
    R: Read + Seek,
{
    type Item = Chunk<'a, R>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.payload_cursor >= self.payload_end {
            None
        } else {
            let chunk = Chunk::from_reader(self.reader, self.payload_cursor);
            self.payload_cursor =
                self.payload_cursor + 4 + 4 + chunk.payload_len + (chunk.payload_len % 2);
            Some(chunk)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Riff<'a, R>
where
    R: Read + Seek,
{
    reader: &'a mut R,
}

#[allow(dead_code)]
impl<'a, R> Riff<'a, R>
where
    R: Read + Seek,
{
    pub fn get_chunk(&mut self) -> Chunk<R> {
        Chunk::from_reader(&mut self.reader, 0)
    }

    pub fn from_file(reader: &'a mut R) -> Self {
        Riff { reader }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunkid_from_str() {
        assert_eq!(ChunkId::new("RIFF").unwrap(), RIFF_ID);
        assert_eq!(ChunkId::new("LIST").unwrap(), LIST_ID);
        assert_eq!(ChunkId::new("seqt").unwrap(), SEQT_ID);

        assert_eq!(
            ChunkId::new("123 ").unwrap(),
            ChunkId {
                value: [0x31, 0x32, 0x33, 0x20]
            }
        );

        assert_eq!(ChunkId::new("123"), None);
        assert_eq!(ChunkId::new("12345"), None);
    }

    #[test]
    fn chunkid_to_str() {
        assert_eq!(RIFF_ID.as_str(), "RIFF");
        assert_eq!(LIST_ID.as_str(), "LIST");
        assert_eq!(SEQT_ID.as_str(), "seqt");
        assert_eq!(ChunkId::new("123 ").unwrap().as_str(), "123 ");
    }
}
