use std::env::VarError;
use ogpt::error::OGptError;

#[derive(Debug)]
pub enum ServerError {
    OGptError(OGptError),
    SerenityError(serenity::Error),
    EnvVarError(VarError)
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