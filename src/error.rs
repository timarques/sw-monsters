use std::fmt::{Display, Debug};
use std::error::Error as DefaultError;

#[derive(Debug, Clone)]
pub struct Error {
    message: String
}

impl Error {

    pub fn new(string: &str) -> Self {
        Self{ message: string.to_string() }
    }

}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl DefaultError for Error {

    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn DefaultError + 'static)> {
        None
    }
}

impl From<reqwest::Error> for Error {

    fn from(error: reqwest::Error) -> Self {
        Error::new(&format!("{}", error))
    }

}

impl From<std::num::ParseIntError> for Error {

    fn from(error: std::num::ParseIntError) -> Self {
        Error::new(&format!("{}", error))
    }

}

impl From<std::io::Error> for Error {

    fn from(error: std::io::Error) -> Self {
        Error::new(&format!("{}", error))
    }

}
