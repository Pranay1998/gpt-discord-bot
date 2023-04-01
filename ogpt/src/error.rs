#[derive(Debug)]
pub enum OGptError {
    Reqwest(reqwest::Error)
}

impl From<reqwest::Error> for OGptError {
    fn from(err: reqwest::Error) -> Self {
        OGptError::Reqwest(err)
    }
}