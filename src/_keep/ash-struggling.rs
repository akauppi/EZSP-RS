/*
* ASH (Asynchronous Serial Host) - version 2 - is Silicon Lab's retry/checkup/wrapper protocol
* for EZSP communications over UART's.
*
* References:
*   - UG101: UART-EZSP Gateway Protocol Reference (Silicon Labs; date unknown)  [1]
*       -> https://www.silabs.com/documents/public/user-guides/ug101-uart-gateway-protocol-reference.pdf
*/
use bitfield_struct::bitfield;

use log::warn;

use some::Some;

//---
// Frame
//
#[derive(Debug, PartialEq)]
enum Frame {
    // Rust (1.76) doesn't allow the enum variants to be types (so that they could have members
    // of their own). Workaround seems to be: declare them as 'struct' outside of the enum (except
    // 'RST' that doesn't need methods). #rust
    //
    //#[cfg(dream)]
    DataFrame{ bytes: /*own*/ Vec<u8>, frmNum: u8, reTx: bool, ackNum: u8 },
    #[cfg(dream)]
    ACK{ /*res: bool,*/ nRdy: bool, ackNum: u8 },
    #[cfg(dream)]
    NAK{ /*res: bool,*/ nRdy: bool, ackNum: u8 },
    RST,
    #[cfg(dream)]
    RSTACK{v: u8, c: u8},       // 'v' always 0x02 (ASH v2)
    #[cfg(dream)]
    ERROR{v: u8, c: u8},         // -''-

    //DataFrame,
    ACK,
    NAK,
    //RST,
    RSTACK,
    ERROR,
}

#[cfg(not(dream))]
//pub struct DataFrame{ bytes: /*own*/ Vec<u8>, frmNum: u8, reTx: bool, ackNum: u8 }
pub struct ACK{ /*res: bool,*/ nRdy: bool, ackNum: u8 }
pub struct NAK{ /*res: bool,*/ nRdy: bool, ackNum: u8 }
//RST,
pub struct RSTACK{v: u8, c: u8}     // 'v' always 0x02 (ASH v2)
pub struct ERROR{v: u8, c: u8}      // -''-

/*** disabled
impl DataFrame {
    fn from( bytes: &[u8], frmNum: u8, reTx: bool, ackNum: u8 ) -> DataFrame {
        //assert!((3..=128_usize).contains(bytes.len()));       // "expected '&usize', got 'usize' #rust
        #[cfg(not(dream))]
        {
            let len = bytes.len();
            assert!(len>=3 && len <=128);
        }
        DataFrame { bytes: bytes.to_vec(), frmNum, reTx, ackNum }
    }
}
***/

/***R
// Rust note: Ideally, not needed, but wasn't able to create an 'ACK' (or 'NAK') in 'Frame::from', without. #help
#[cfg(no)]
impl ACK {
    fn new(nRdy: bool, ackNum: u8) -> Self {
        Self { nRdy, ackNum }
    }
}

impl ACK {      // #rust: this compiles; a separate 'fn ACK_gen(cb: u8) -> Frame' didn't. Why??
    fn gen(cb: u8) -> Self {
        let z = CB_ACK::from(cb);
        Self{ nRdy: z.nRdy(), ackNum: z.ackNum() }
    }
}

impl NAK {
    fn new(nRdy: bool, ackNum: u8) -> Self {
        Self { nRdy, ackNum }
    }
}

impl RSTACK {
    fn from(data: &[u8;2]) -> Self {
        if data[0] != 0x02 {
            warn!("Unexpected version: {} != 0x02", data[0]);
        };
        Self{v: data[0], c: data[1]}
    }
}

impl ERROR {
    fn from(data: &[u8;2]) -> Self {
        if data[0] != 0x02 {
            warn!("Unexpected version: {} != 0x02", data[0]);
        };
        Self{v: data[0], c: data[1]}
    }
}
***/

impl Frame {
    /**
    * Create an ASH 'Frame' from raw data. The raw data is an array (4..132 bytes long), ending
    * in '0x7E' ("flag byte") and not having that byte elsewhere within it.
    *
    * i.e. termination of the frame, CRC checking and interpreting the frame type is done here
    */
    fn from(raw: &[u8]) -> Result<Frame,String> {
        let len: usize = raw.len();

        (len < 4)   .some().ok_or_else(|| format!("too short: {len} < 4"))?;
        (len > 13)  .some().ok_or_else(|| format!("too long: {len} > 132"))?;

        let [cb, .., a, b, fb] = *raw;

        // check Flag Byte
        (fb == 0x7e)
            .some().ok_or_else(|| format!("no flag byte at the end: {fb} != 0x7e"))?;

        // check CRC
        let crc: u16 = u16::from_be_bytes([a,b]);
        let crc2: u16 = calc_crc(&raw[1..len-3]);
        (crc == crc2)
            .some().ok_or_else(|| format!("CRC mismatch: {crc} != {crc2}"))?;

        // differentiate based on the frame type
        let fr: Frame = match cb {
            x if x&0x80 == 0 => {       // data
                //let (frmNum, reTx, ackNum): (u8,bool,u8) = (cb >> 4&0x07, cb>>3&0x01 != 0, cb>>0&0x07);

                //let [ackNum, reTx, frmNum] = bitfields_extract(cb, [3,1,3]);
                //let reTx: bool = reTx != 0;

                //let z = CB_DataFrame::from(x & 0x7f);
                //let CB_DataFrame{ ackNum, reTx, frmNum, .. } = z;   // "does not contain field 'frmNum'"
                //  ^-- the 'bitstruct_field' doesn't create fields but accessor functions

                let (frmNum, reTx, ackNum) = {
                    let z = CB_DataFrame::from(x&0x7f);
                    (z.frmNum(), z.reTx(), z.ackNum())
                };

                Self::DataFrame{ bytes: raw[1..len-3].to_vec(), frmNum, reTx, ackNum }
            },
            #[cfg(tbd)]
            x if x&0xe0 == 0x80 => {    // ACK
                let (nRdy, ackNum) = {
                    let z = CB_ACK::from(x&0x1f);
                    (z.nRdy(), z.ackNum())
                };
                // /*Frame::*/ACK{ nRdy, ackNum }  // "expected 'Frame', found 'ACK' #rust
                //Self::ACK{ nRdy, ackNum }        // "Variant 'Frame::ACK' has no field named 'nRdy'
                ACK::new(nRdy, ackNum)              // Q: Why does this work, but just dressing 'ACK' doesn't? (this gives 'Frame' type, but why?)
            },
            x if x&0xe0 == 0x80 =>    // ACK
                ACK::gen(x&0x1f),       // "expected 'Frame', found 'ACK'"

            #[cfg(tbd)]
            x if x&0xe0 == 0xa0 => {    // NAK
                let (nRdy, ackNum) = {
                    let z = CB_NAK::from(x&0x1f);
                    (z.nRdy(), z.ackNum())
                };
                /*Frame::*/NAK{ nRdy, ackNum }
            },
            0xc0 => Frame::RST,
            0xc1 => /*Frame::*/RSTACK::from(&[raw[1],raw[2]]), // Rust note: '&raw[1..=2]' wouldn't provide needed '&[u8;2]' type
            0xc2 => /*Frame::*/ERROR::from(&[raw[1],raw[2]]),

            _ => unreachable!()
        };
        Ok(fr)
    }

    fn out(/*gulp*/ self) -> &[u8] {


        unimplemented!()
    }
}

//---
// Control Byte layouts
//
#[bitfield(u8)]
#[allow(non_camel_case_types)]
struct CB_DataFrame {
    #[bits(3)]
    pub ackNum: u8,     // b0..2
    pub reTx: bool,     // b3
    #[bits(3)]
    pub frmNum: u8,     // b4..6
    __: bool
}

#[bitfield(u8)]
#[allow(non_camel_case_types)]
struct CB_ACK {
    #[bits(3)]
    pub ackNum: u8,     // b0..2
    pub nRdy: bool,     // b3
    pub res: bool,      // b4
    #[bits(3)]
    __: u8
}
#[allow(non_camel_case_types)]
type CB_NAK = CB_ACK;

/**R
//---
// Frame
//
#[derive(Debug, PartialEq)]
pub struct _Frame{
    control: u8,
    data: Option<_DataField>,
    crc: u16,
    flag: _FlagByte
}

#[derive(Debug, PartialEq)]
struct _DataField(/*own*/ [u8]);
    // Data length: 2..128 bytes

#[derive(Debug, PartialEq)]
struct _FlagByte(u8);
**/

fn calc_crc(bytes: &[u8]) -> u16 {

    //unimplemented!();
    0x9b7b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc() {
        let d = &[0xc1, 0x02, 0x02];       // from [1]
        assert_eq!(calc_crc(&d), 0x9b7b);           // -''-
    }

    #[test]
    fn RST_out() {
        let f = Frame::RST;
        assert_eq!(f.out(), &[0xc0, 0x38, 0xbc, 0x7e]);     // array from [1]
    }

    // Note: no test for "RST in"; host only

    // Note: no test for "RSTACK out"; NCP only

    #[test]
    fn RSTACK_in() {
        let f = Frame::from(&[0xc1, 0x02, 0x03, 0x9b, 0x7b, 0x7e]);
            // slightly changed array from [1], to not use same value for 'v' and 'c'
        assert_eq!(f, Frame::RSTACK{v: 0x02, c: 0x03});
    }

    //... tbd. ERROR
}
