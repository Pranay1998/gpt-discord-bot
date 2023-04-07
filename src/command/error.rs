use std::fmt;

#[derive(Debug)]
pub struct CommandError {
    command: String,
    error_message: String
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error while handling {} - {}", self.command, self.error_message)
    }
}

impl std::error::Error for CommandError {}

impl CommandError {
    pub fn new(command: String, error_message: String) -> Self {
        Self {
            command,
            error_message
        }
    }
}