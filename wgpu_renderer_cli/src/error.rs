use std::convert::From;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Mesh(mesh::Error),
    Renderer(wgpu_renderer::Error),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    Image(image::ImageError),
    ImageContainerTooSmall,
    EmptyMesh,
    ZeroAreaImage,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "Network error: {:?}", e),
            Error::Mesh(e) => write!(f, "Error building mesh: {:?}", e),
            Error::Renderer(e) => write!(f, "Renderer error: {:?}", e),
            Error::ParseInt(e) => write!(f, "Error parsing integer: {:?}", e),
            Error::ParseFloat(e) => write!(f, "Error parsing float: {:?}", e),
            Error::Image(e) => write!(f, "Error performing image operation: {:?}", e),
            Error::ImageContainerTooSmall => {
                write!(f, "The container for the image data is too small.")
            }
            Error::EmptyMesh => write!(f, "Mesh is empty."),
            Error::ZeroAreaImage => write!(f, "Image has an area of zero."),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Mesh(e) => Some(e),
            Error::Renderer(e) => Some(e),
            Error::ParseInt(e) => Some(e),
            Error::ParseFloat(e) => Some(e),
            Error::Image(e) => Some(e),
            Error::ImageContainerTooSmall => None,
            Error::EmptyMesh => None,
            Error::ZeroAreaImage => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<mesh::Error> for Error {
    fn from(error: mesh::Error) -> Self {
        Error::Mesh(error)
    }
}

impl From<wgpu_renderer::Error> for Error {
    fn from(error: wgpu_renderer::Error) -> Self {
        Error::Renderer(error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParseInt(error)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(error: std::num::ParseFloatError) -> Self {
        Error::ParseFloat(error)
    }
}

impl From<image::ImageError> for Error {
    fn from(error: image::ImageError) -> Self {
        Error::Image(error)
    }
}
