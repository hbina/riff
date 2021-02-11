pub mod constants;
pub mod error;
pub mod fourcc;
pub mod riffu;

pub use error::RiffError;
pub use fourcc::FourCC;
pub use riffu::{Chunk, ChunkIter, Riff};
