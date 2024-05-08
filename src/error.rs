/// A convenience type alias for [`Result`](std::result::Result) with the error variant as [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

// TODO: this good?
/// The rustmatica error type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Any NBT error.
    #[error(transparent)]
    Nbt(#[from] fastnbt::error::Error),

    /// Any IO error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
