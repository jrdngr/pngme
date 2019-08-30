use std::fmt;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "Something went wrong. Oh no!"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}
