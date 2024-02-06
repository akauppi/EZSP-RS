use std::env;
use serialport;

use anyhow::Result;
use log::info;

use std::time::Duration;

//static out: [u8] = [0,0,0,8];

const DEV_PATH: &str ="/dev/ttyACM0";

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();

    if true {
        let ports = serialport::available_ports().expect("No ports found!");
        for p in ports {
            println!("{}", p.port_name);
        }
    }

    let port = serialport::new(DEV_PATH, 115_200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    info!("Port opened: {port:?}");

    Ok(())
}
