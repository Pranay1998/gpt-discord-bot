use std::{env::VarError, fmt, error};
use ogpt::error::OGptError;

use crate::command::CommandError;

#[derive(Debug)]
pub enum ServerError {
    CommandError(CommandError),
    OGptError(OGptError),
    SerenityError(serenity::Error),
    EnvVarError(VarError)
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerError::OGptError(err) => write!(f, "OGpt error: {}", err),
            ServerError::SerenityError(err) => write!(f, "Serenity error: {}", err),
            ServerError::EnvVarError(err) => write!(f, "Env var error: {}", err),
            ServerError::CommandError(err) => write!(f, "Command error: {}", err.to_string()),
        }
    }
}

impl From<OGptError> for ServerError {
    fn from(err: OGptError) -> Self {
        ServerError::OGptError(err)
    }
}

impl From<serenity::Error> for ServerError {
    fn from(err: serenity::Error) -> Self {
        ServerError::SerenityError(err)
    }
}

impl From<VarError> for ServerError {
    fn from(err: VarError) -> Self {
        ServerError::EnvVarError(err)
    }
}

impl From<CommandError> for ServerError {
    fn from(err: CommandError) -> Self {
        ServerError::CommandError(err)
    }
}

impl error::Error for  ServerError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            ServerError::EnvVarError(err) => Some(err),
            ServerError::OGptError(err) => Some(err),
            ServerError::SerenityError(err) => Some(err),
            ServerError::CommandError(err) => Some(err),
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ServerError::OGptError(err) => err.source(),
            ServerError::SerenityError(err) => err.source(),
            ServerError::EnvVarError(err) => err.source(),
            ServerError::CommandError(err) => err.source(),
        }
    }
}