use std::convert::From;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Nom(nom_stl::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Nom(e) => write!(f, "Nom STL error: {:?}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Nom(e) => Some(e),
        }
    }
}

impl From<nom_stl::Error> for Error {
    fn from(error: nom_stl::Error) -> Self {
        Error::Nom(error)
    }
}
