pub mod constants;
pub mod error;
pub mod fourcc;
pub mod riff;

pub use error::RiffError;
pub use fourcc::FourCC;
pub use riff::{Chunk, ChunkIter, Riff};
