use std::io::{Read, Seek};
use std::rc::Rc;

pub const RIFF_ID: &str = "RIFF";
pub const LIST_ID: &str = "LIST";
pub const SEQT_ID: &str = "SEQT";

#[derive(Debug)]
pub struct ChunkType {
    pub value: [u8; 4],
}

impl ChunkType {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.value).unwrap()
    }
}

#[derive(Debug)]
pub struct ChunkId {
    pub value: [u8; 4],
}

impl ChunkId {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.value).unwrap()
    }
}

/// Lazy version of `ChunkId`.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ChunkIdDisk<R>
where
    R: Read + Seek,
{
    pos: u32,
    reader: Rc<R>,
}

impl<R> ChunkIdDisk<R>
where
    R: Read + Seek,
{
    pub fn as_string(&mut self) -> std::io::Result<String> {
        let pos = self.pos as u64;
        let mut str_buff: [u8; 4] = [0; 4];
        let reader = Rc::get_mut(&mut self.reader).unwrap();
        reader.seek(std::io::SeekFrom::Start(pos))?;
        reader.read_exact(&mut str_buff)?;
        // TODO: I think we need to introduce a custom error type because of this.
        // This is a `std::str::Utf8Error` type.
        Ok(String::from_utf8(Vec::from(str_buff)).unwrap())
    }

    pub fn new(reader: Rc<R>, pos: u32) -> ChunkIdDisk<R> {
        ChunkIdDisk {
            pos,
            reader: reader.clone(),
        }
    }
}
