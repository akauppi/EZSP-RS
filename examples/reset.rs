use std::env;
use serialport;
use serialport::{ClearBuffer, TTYPort, FlowControl, SerialPort};
    // 'SerialPort' needed for '.close' to be possible for 'TTYPort'

use anyhow::Result;
use log::info;
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

    {
        let ports = serialport::available_ports().expect("No ports found!");
        for p in ports {
            println!("{}", p.port_name);
        }
    }

    info!("Opening {DEV_PATH}...");

    // Note: Don't care about Windows-side compatibility; simpler to do 'open_native()': returns
    //      unboxed 'TTYPort'.
    //
    let mut port = serialport::new(DEV_PATH, 115_200)
        .flow_control(FlowControl::Hardware)    // 8N1 are default
            //
        .timeout(Duration::from_millis(10))
        .open_native()?;

    port.clear(ClearBuffer::All)?;

    #[allow(non_snake_case)]
    fn send_RST(port: &mut TTYPort) -> Result<()> {        // Rust Q: why not 'impl SerialPort'????
        const RST_FRAME: &'static [u8] = &[0xc0, 0x38, 0xbc, 0x7e];

        port.write(RST_FRAME)?;
        Ok(())
    }

    /*
    * Wait 1..2s and print anything that comes in.
    *
    * C1 02 02 9B 7B 7E
    */
    fn print_any_response(port: &mut TTYPort) -> Result<()> {
        let mut v = vec![0;32];
        port.read( v.as_mut_slice() )?;
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

        send_RST(&mut port)?;   // "The trait bound 'Box<dyn SerialPort>: SerialPort' is not satisfied

        print_any_response(&mut port)?;

        //info!("Retry {retry_round+1}...");  // Rust doesn't allow
        { let x=retry_round+1; info!("Retry {x}..."); }
    }

    // That's it - we should be connected.

    Ok(())
}
