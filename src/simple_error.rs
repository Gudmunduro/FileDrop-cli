use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct SimpleError {
    pub message: String,
}

impl SimpleError {
    pub fn new(messsage: String) -> SimpleError {
        SimpleError { message: messsage }
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SimpleError {
    fn description(&self) -> &str {
        &self.message
    }
}