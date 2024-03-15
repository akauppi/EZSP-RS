mod protocol;
mod device;

pub struct EZSP;

impl EZSP {
    pub fn new<'a>(_path: impl Into<std::borrow::Cow<'a, str>>) -> Result<EZSP,std::io::Error> {    // <-- tbd. that 'Error' shall change
        // Rust note: I don't claim to understand why the above 'path' prototype is such. Comes from
        //      'serialport' crate's '::new'.

        // nada. tbd.
        Ok(EZSP)
    }

    #[allow(non_snake_case)]
    pub async fn getRandomNumber(&self) -> u16 {

        0x0
        // !unimplemented!()
    }
}
