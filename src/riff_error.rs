use std::fmt::{Debug, Formatter};
use std::io::Error;

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
