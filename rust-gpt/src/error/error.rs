#[derive(Debug)]
pub enum RustGptError {
    Reqwest(reqwest::Error)
}

impl From<reqwest::Error> for RustGptError {
    fn from(err: reqwest::Error) -> Self {
        RustGptError::Reqwest(err)
    }
}