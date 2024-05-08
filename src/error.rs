pub type Result<T> = std::result::Result<T, Error>;

// TODO: this good?
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Nbt(#[from] fastnbt::error::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
