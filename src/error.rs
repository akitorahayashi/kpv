use std::error::Error;
use std::fmt::{self, Display};
use std::io;

/// Library-wide error type capturing domain-specific and underlying I/O failures.
#[derive(Debug)]
pub enum KpvError {
    Io(io::Error),
    HomeNotConfigured,
    EnvFileNotFound,
    KeyNotFound(String),
    EnvAlreadyExists,
}

impl Display for KpvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KpvError::Io(err) => write!(f, "{}", err),
            KpvError::HomeNotConfigured => write!(f, "HOME environment variable not set"),
            KpvError::EnvFileNotFound => {
                write!(f, "No .env file found in the current directory")
            }
            KpvError::KeyNotFound(key) => write!(f, "No .env file found for key '{}'", key),
            KpvError::EnvAlreadyExists => {
                write!(f, ".env file already exists in the current directory")
            }
        }
    }
}

impl Error for KpvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            KpvError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for KpvError {
    fn from(value: io::Error) -> Self {
        KpvError::Io(value)
    }
}

impl KpvError {
    pub(crate) fn key_not_found<S: Into<String>>(key: S) -> Self {
        KpvError::KeyNotFound(key.into())
    }

    /// Provide an `io::ErrorKind`-like view for callers expecting legacy behavior.
    pub fn kind(&self) -> io::ErrorKind {
        match self {
            KpvError::Io(err) => err.kind(),
            KpvError::HomeNotConfigured => io::ErrorKind::NotFound,
            KpvError::EnvFileNotFound => io::ErrorKind::NotFound,
            KpvError::KeyNotFound(_) => io::ErrorKind::NotFound,
            KpvError::EnvAlreadyExists => io::ErrorKind::AlreadyExists,
        }
    }
}
