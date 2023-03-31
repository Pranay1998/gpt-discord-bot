use std::env::VarError;
use rust_gpt::error::RustGptError;

#[derive(Debug)]
pub enum ServerError {
    GptError(RustGptError),
    SerenityError(serenity::Error),
    EnvVarError(VarError)
}

impl From<RustGptError> for ServerError {
    fn from(err: RustGptError) -> Self {
        ServerError::GptError(err)
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