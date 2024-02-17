use std::env;
use serialport;
use serialport::ClearBuffer;

use anyhow::Result;
use log::info;

use std::time::Duration;

const OUT: &'static [u8] = &[0,0,0,13];

const DEV_PATH: &str ="/dev/ttyACM0";

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

    info!("A");

    let mut port = serialport::new(DEV_PATH, 115_200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    info!("Port opened: {port:?}");

    port.clear(ClearBuffer::Input)?;

    let output = OUT;   //"This is a test. This is only a test.".as_bytes();
    port.write(output).expect("Write failed!");

    info!("Written to");

    let mut v = vec![0; 32];
    port.read( v.as_mut_slice()).expect("Found no data!");

    Ok(())
}
