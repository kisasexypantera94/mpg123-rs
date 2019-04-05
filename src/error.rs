use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    EOF,
    BadInput,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::EOF => "End of reader",
            Error::BadInput => "Bad input",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}
