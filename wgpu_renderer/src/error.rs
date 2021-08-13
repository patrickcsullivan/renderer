use std::convert::From;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    BufferAsync(wgpu::BufferAsyncError),
    RequestDevice(wgpu::RequestDeviceError),
    NoAdapter,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "Network error: {:?}", e),
            Error::BufferAsync(e) => write!(f, "Buffer async error: {:?}", e),
            Error::RequestDevice(e) => write!(
                f,
                "No connection to a physical graphics or compute device: {:?}",
                e
            ),
            Error::NoAdapter => write!(f, "No adapter to a physical graphics or compute device"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::BufferAsync(e) => Some(e),
            Error::RequestDevice(e) => Some(e),
            Error::NoAdapter => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<wgpu::BufferAsyncError> for Error {
    fn from(error: wgpu::BufferAsyncError) -> Self {
        Error::BufferAsync(error)
    }
}

impl From<wgpu::RequestDeviceError> for Error {
    fn from(error: wgpu::RequestDeviceError) -> Self {
        Error::RequestDevice(error)
    }
}
