#[derive(Debug)]
pub enum RiffError {
    Io(std::io::Error),
    ByteLessThan8(usize),
    PayloadLenMismatch(usize, u32),
}

impl From<std::io::Error> for RiffError {
    fn from(v: std::io::Error) -> Self {
        RiffError::Io(v)
    }
}
