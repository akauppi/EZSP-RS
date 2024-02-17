// Based on -> https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub(crate) struct MyError {
    details: String
}

impl MyError {
    pub(crate) fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[cfg(test)]
mod tests {
    use super::MyError;

    fn raises_my_error(yes: bool) -> Result<(), MyError> {
        if yes {
            Err(MyError::new("borked"))
        } else {
            Ok(())
        }
    }
}