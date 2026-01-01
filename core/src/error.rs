#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AeonError {
    InvalidInput,
    MathError,
    InsufficientData,
    CryptoError,
}
pub type Result<T> = core::result::Result<T, AeonError>;
