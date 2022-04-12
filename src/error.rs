use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NBT(fastnbt::error::Error),
    IO(std::io::Error),
}
impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::NBT(e) => e.to_string(),
            Self::IO(e) => e.to_string(),
        })
    }
}

impl From<fastnbt::error::Error> for Error {
    fn from(err: fastnbt::error::Error) -> Self {
        Self::NBT(err)
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}
