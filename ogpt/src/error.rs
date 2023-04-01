use std::{error, fmt};

#[derive(Debug)]
pub enum OGptError {
    Reqwest(reqwest::Error),
    SerdeJsonError(serde_json::Error)
}

impl fmt::Display for OGptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OGptError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            OGptError::SerdeJsonError(err) => write!(f, "Serde json error: {}", err),
        }
    }
}

impl From<reqwest::Error> for OGptError {
    fn from(err: reqwest::Error) -> Self {
        OGptError::Reqwest(err)
    }
}

impl From<serde_json::Error> for OGptError {
    fn from(err: serde_json::Error) -> Self {
        OGptError::SerdeJsonError(err)
    }
}


impl error::Error for OGptError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            OGptError::Reqwest(err) => Some(err),
            OGptError::SerdeJsonError(err) => Some(err)
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            OGptError::Reqwest(err) => err.source(),
            OGptError::SerdeJsonError(err) => err.source(),
        }
    }
}