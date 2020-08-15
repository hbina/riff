use std::fmt::{Debug, Display, Formatter};
use std::io::{Error, Read, Seek};
use std::rc::Rc;

pub enum RiffError {
    IoError(std::io::Error),
    LibError(&'static str),
}

impl From<std::io::Error> for RiffError {
    fn from(v: Error) -> Self {
        RiffError::IoError(v)
    }
}

impl Debug for RiffError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
