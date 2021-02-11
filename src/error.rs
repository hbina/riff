#[derive(Debug)]
pub enum RiffError {
    InsufficientBytes,
    Other(Box<dyn std::error::Error>),
}

macro_rules! impl_error {
    ( $error : ty ) => {
        impl From<$error> for RiffError {
            /// Performs the conversion.
            fn from(v: $error) -> Self {
                RiffError::Other(Box::new(v))
            }
        }
    };
}

impl_error!(std::array::TryFromSliceError);
impl_error!(std::io::Error);

/// A convenient `Result` type.
pub type RiffResult<T> = Result<T, RiffError>;
