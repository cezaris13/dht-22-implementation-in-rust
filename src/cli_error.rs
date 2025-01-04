use std::error;
use std::fmt;

#[derive(Debug)]
pub enum CliError {
    Error(String),
    IOError(std::io::Error),
    GPIOError(rppal::gpio::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Error(err) => write!(f, "Error in program: {}", err),
            Self::IOError(err) => write!(f, "IO error in program: {}", err),
            Self::GPIOError(err) => write!(f, "GPIO error in program: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::IOError(err) => Some(err),
            Self::GPIOError(err) => Some(err),
            Self::Error(_) => None,
        }
    }
}

macro_rules! from_error {
    ($source_error:ty, $target_error:ident::$variant:ident) => {
        impl From<$source_error> for $target_error {
            fn from(err: $source_error) -> $target_error {
                $target_error::$variant(err)
            }
        }
    };
}

from_error!(String, CliError::Error);
from_error!(std::io::Error, CliError::IOError);
from_error!(rppal::gpio::Error, CliError::GPIOError);