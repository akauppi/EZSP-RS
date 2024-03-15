/*
* Port
*
* Byte level access to the device.
*
* Reading is exposed as a 'Stream' (async); writing as a synchronous (blocking; all at once) call.
*
* NOTE!
*   THE AUTHOR IS ONLY A NOVICE IN RUST PROGRAMMING. If you think something can be expressed better,
*   please chime 🛎️ in and leave a message! :)
*/
use log::debug;
use serialport::{self, TTYPort};

use futures_core::stream::Stream;       // aka futures::stream::Stream

use core::{pin::Pin, task::{Context, Poll}};        // Q: not in 'std::prelude'?
use std::io::{self, Read as _};

pub struct Port {
    port: TTYPort
}

impl Port {
    fn new(port: TTYPort) -> Self {     // takes ownership of the port
        Port{
            port
        }
    }
}

// Rust 'futures' note: The 'Stream' we provide is never-ending. The 'futures' API could make a
//      distinction with a 'Stream' that can end; and one that doesn't.
//
impl /*InfiniteFailable*/ Stream for Port {
    type Item = u8;     // tbd. can consider '[u8]'

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buf = [0;1];

        debug!("Reading a u8");

        match self.get_mut().port.read(&mut buf) {
            Ok(0) => Poll::Pending,
            Ok(len) => Poll::Ready(Some(1)),
            Err(err) =>
                panic!("Cannot read from device: {}", err)    // tbd. make 'FailableStream' and allow the app to get this
        }
    }

    //fn size_hint(&self) -> (usize, Option<usize>)     // not implemented; "[...] bounds on the remaining length of the stream."
}

impl io::Write for Port {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.port.write(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.port.flush();
        Ok(())
    }
}
