/*
* ASH (Asynchronous Serial Host) - version 2 - is Silicon Lab's retry/checkup/wrapper protocol
* for EZSP communications over UART's.
*
* References:
*   - UG101: UART-EZSP Gateway Protocol Reference (Silicon Labs; date unknown)  [1]
*       -> https://www.silabs.com/documents/public/user-guides/ug101-uart-gateway-protocol-reference.pdf
*/
#![allow(non_snake_case)]

use bitfield_struct::bitfield;

//use log::warn;

use some::Some;

//---
// Frame
//
// RUST NOTE!!!
//
//  In Rust, enum "variants" are not types (as of Rust 1.76). This creates all kinds of.. issues
//  if approaching them as if they were.
//
//  The author tries to solve this with:
//      - define all variants inside the 'enum' type; not using separate 'struct' (they're essentially
//        there so that same type could be used in multiple enum's, but we don't need that).
//        (( The problem 'struct' gave was: how to create a 'Frame' from an outside struct. ))
//
//      - all creation of variants needs to happen from within 'impl Frame'.
//
#[derive(Debug, PartialEq)]
enum Frame {
    DataFrame{ bytes: /*own*/ Vec<u8>, frmNum: u8, reTx: bool, ackNum: u8 },
    ACK{ /*res: bool,*/ nRdy: NRdy, ackNum: u8 },
    NAK{ /*res: bool,*/ nRdy: NRdy, ackNum: u8 },
    RST,
    RSTACK{v: u8, c: u8},       // 'v' always 0x02 (ASH v2)
    ERROR{v: u8, c: u8},        // -''-
}

// Rust: There's no way to add, say, 'from( bytes: &[u8], ... )' to 'DataFrame', now that it's
//      defined as a variant. That's a pity.    | "Variants are not types" in Rust.
//
/*impl Frame::DataFrame {
    fn from() -> Self { unimplemented!() }
}*/

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum NRdy {
    Ready = 0,
    NotReady = 1
}
impl NRdy {
    // These have to be const fn's
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Ready,
            1 => Self::NotReady,
            _ => panic!("not suitable 'NRdy' value")     // Rust note: must be const string, since const fn
        }
    }
}

impl Frame {
    /**
    * Create an ASH 'Frame' from raw data. The raw data is an array (4..132 bytes long), ending
    * in '0x7E' ("flag byte") and not having that byte elsewhere within it.
    *
    * i.e. termination of the frame, CRC checking and interpreting the frame type is done here
    */
    pub fn from(raw: &[u8]) -> Result<Frame,String> {
        let len: usize = raw.len();

        (len < 4)   .some().ok_or_else(|| format!("too short: {len} < 4"))?;
        (len > 13)  .some().ok_or_else(|| format!("too long: {len} > 132"))?;

        let [cb, .., a, b, fb] = *raw
            else { panic!() };

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

            x if x&0xe0 == 0x80 => {   // ACK
                let z = CB_ACK::from(x & 0x1f);
                Self::ACK{ nRdy: z.nRdy(), ackNum: z.ackNum() }
            },

            x if x&0xe0 == 0xa0 => {  // NAK
                let z = CB_NAK::from(x & 0x1f);
                Self::NAK{ nRdy: z.nRdy(), ackNum: z.ackNum() }
            },

            0xc0 => Self::RST,
            0xc1 => Self::RSTACK{ v: raw[1], c: raw[2]},
            0xc2 => Self::ERROR{ v: raw[1], c: raw[2]},

            _ => unreachable!()
        };
        Ok(fr)
    }

    fn out(/*gulp*/ self) -> Vec<u8> {
        unimplemented!()
    }
}

// Control Byte layouts
//
// Note: Fields can be private; they are only used within this module. Also, they generate methods,
//      the fields are just optical illusion...
//
#[bitfield(u8)]
#[allow(non_camel_case_types)]
struct CB_DataFrame {
    #[bits(3)]
    ackNum: u8,     // b0..2
    reTx: bool,     // b3
    #[bits(3)]
    frmNum: u8,     // b4..6
    __: bool
}

#[bitfield(u8)]
#[allow(non_camel_case_types)]
struct CB_ACK {
    #[bits(3)]
    ackNum: u8,     // b0..2
    #[bits(1)]
    nRdy: NRdy,     // b3
    res: bool,      // b4
    #[bits(3)]
    __: u8
}
#[allow(non_camel_case_types)]
type CB_NAK = CB_ACK;

fn calc_crc(_bytes: &[u8]) -> u16 {

    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc() {
        let d: &[u8] = &[0xc1, 0x02, 0x02];         // from [1]
        assert_eq!(calc_crc(&d), 0x9b7b);           // -''-
    }

    // Note: no test for "RST in"; host only
    #[test]
    fn RST_out() {
        let f = Frame::RST;
        assert_eq!(f.out(), &[0xc0, 0x38, 0xbc, 0x7e]);     // array from [1]
    }

    // Note: no test for "RSTACK out"; NCP only
    #[test]
    fn RSTACK_in() -> Result<(), String> {
        let f = Frame::from(&[0xc1, 0x02, 0x03, 0x9b, 0x7b, 0x7e])?;
            // slightly changed array from [1], to not use same value for 'v' and 'c'
        assert_eq!(f, Frame::RSTACK{v: 0x02, c: 0x03});
        Ok(())
    }

    // Note: no test for "ERROR out"; NCP only
    #[test]
    fn ERROR_in() -> Result<(), String> {
        let f = Frame::from(&[0xc2, 0x01, 0x52, 0xfa, 0xbd, 0x7e])?;    // from [1]
        assert_eq!(f, Frame::ERROR{v: 0x01, c: 0x52});
        Ok(())
    }

    #[test]
    fn DATA_out() -> Result<(), String> {   // [1]: "version" command, no pseudo-random number
        let f = Frame::DataFrame { bytes: vec![0,0,0,2], frmNum: 2, ackNum: 5, reTx: false };
        assert_eq!(f.out(), &[0x25, 0x00, 0x00, 0x00, 0x02, 0x1a, 0xad, 0x7e]);
        Ok(())
    }

    #[test]
    fn DATA_in() -> Result<(), String> {    // [1]: "version" response, no pseudo-random number
        let d: &[u8] = &[0x00, 0x80, 0x00, 0x02, 0x02, 0x11, 0x30];
        let f = Frame::from(d)?;
        assert_eq!(f, Frame::DataFrame{ bytes: d.to_vec(), frmNum: 5, ackNum: 3, reTx: false });
        Ok(())
    }

    #[test]
    fn ACK_out() -> Result<(), String> {   // [1]
        use NRdy::Ready;
        let f = Frame::ACK { ackNum: 1, nRdy: Ready };
        assert_eq!(f.out(), &[0x81, 0x60, 0x59, 0x7e]);
        Ok(())
    }

    #[test]
    fn ACK_in() -> Result<(), String> {    // [1]
        use NRdy::NotReady;
        let f = Frame::from(&[0x8e, 0x91, 0xb6, 0x7e])?;
        assert_eq!(f, Frame::ACK{ ackNum: 6, nRdy: NotReady });
        Ok(())
    }

    #[test]
    fn NAK_out() -> Result<(), String> {   // [1]
        use NRdy::Ready;
        let f = Frame::NAK { ackNum: 6, nRdy: Ready };
        assert_eq!(f.out(), &[0xa6, 0x34, 0xdc, 0x7e]);
        Ok(())
    }

    #[test]
    fn NAK_in() -> Result<(), String> {    // [1]
        use NRdy::NotReady;
        let f = Frame::from(&[0xad, 0x85, 0xb7, 0x7e])?;
        assert_eq!(f, Frame::NAK{ ackNum: 5, nRdy: NotReady });
        Ok(())
    }
}
