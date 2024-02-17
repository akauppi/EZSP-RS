
use std::error::Error;

mod error;
use error::MyError;

pub trait OrBust where Self: Into<bool> {
    #[allow(non_snake_case)]
    fn or_bust<E,F>(&self, errGen: F) -> Result<(), Box<dyn Error>>
        where
            F: FnOnce() -> E,
            E: std::error::Error
    ;
}

impl OrBust for bool {
    #[allow(non_snake_case)]
    fn or_bust<E,F>(&self, errGen: F) -> Result<(), Box<dyn std::error::Error>>
        where
            F: FnOnce() -> E,
            E: std::error::Error
    {
        if *self == true { Ok(()) }
        else { Err(Box::new(errGen())) }
    }
}

#[cfg(test)]
mod tests {
    use crate::OrBust;

    #[test]
    fn jug() {
        (1 < 2) .or_bust(|| "We are bust.".into() )?;
        (2 > 1) .or_bust(|| "Should be an error.".into() )?;
    }
}
