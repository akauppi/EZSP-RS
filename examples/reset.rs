use std::env;
use serialport::{self, ClearBuffer, TTYPort, SerialPort};
    // 'SerialPort' needed for '.close' to be possible for 'TTYPort'

use anyhow::Result;
use log::{debug, info};
use std::io::{Read, Write};

use std::time::Duration;

const DEV_PATH: &str ="/dev/ttyACM0";

// tbd. Eventually, this becomes part of the library and every example uses it from there.

/*
* Reset communications on the ASH level.
*
* These are the steps (described in [1]), how to set the NCP to "connection" state.
*
* References:
*   - "UG101: UART-EZSP Gateway Protocol Reference" [1]
*       -> https://www.silabs.com/documents/public/user-guides/ug101-uart-gateway-protocol-reference.pdf
*/
fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();

    info!("Opening {DEV_PATH}...");

    // Note: Don't care about Windows compatibility; simpler to use 'TTYPort' without abstractions.
    //
    let mut port = serialport::new(DEV_PATH, 115_200)
        //.flow_control(FlowControl::Hardware)    // 8N1 are default
            //
        .timeout(Duration::from_millis(500 /*10*/))
        .open_native()?;

    info!("Clearing the state...");
    port.clear(ClearBuffer::All)?;

    #[allow(non_snake_case)]
    fn send_RST(port: &mut TTYPort) -> Result<()> {
        let RST_FRAME: &[u8] = &[0xc0, 0x38, 0xbc, 0x7e];

        debug!("Sending: {RST_FRAME:x?}");
        port.write(RST_FRAME)?;
        Ok(())
    }

    /*
    * Wait 1..2s and print anything that comes in.
    *
    * C1 02 02 9B 7B 7E
    */
    fn print_any_response(port: &mut TTYPort) -> Result<()> {
        debug!("Listening...");

        // Note! For 'io::Read', the buffer needs to be initialized before reads.
        //
        //      "It is your responsibility to make sure that buf is initialized before calling read"
        //          -> https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read
        //
        let mut buf = [0;200];

        loop {
            let n = port.read(&mut buf).unwrap_or_else(|e| {
                debug!("{e:?}");
                0
            });

            {
                let read_slice = &buf[0..n];
                debug!("Received {n} bytes: {read_slice:x?}");  // tbd. pad with 0's
            }
        }
        Ok(())
    }

    // Sending ASH 'RST' cancels any pending frame reception in the ASH.
    // It shall reply with 'RSTACK'.
    // If that's not heard within a certain time, the host should retry 5 times.
    //
    //  ^-- tbd. Check with [1], when moving to lib.
    //
    info!("Sending RST...");

    for retry_round in 1.. {
        if retry_round == 6 {
            // Rust: would like to return "some" 'Err::Error' but didn't have the patience to
            //      figure it out.
            //
            let n= retry_round-1;
            eprintln!("RST / RSTACK did not happen, despite {n} tries.");
            std::process::exit(1);
        }

        send_RST(&mut port)?;

        print_any_response(&mut port)?;

        //info!("Retry {retry_round+1}...");  // Rust doesn't allow
        { let x=retry_round+1; info!("Retry {x}..."); }
    }

    // That's it - we should be connected.

    Ok(())
}
