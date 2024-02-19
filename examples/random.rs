use std::env;

use anyhow::Result;
use log::{info};

use std::time::Duration;

use ezsp_sample::EZSP;

const DEV_PATH: &str ="/dev/ttyACM0";

const ZZZ: Duration = Duration::from_millis(500);

/*
* This example gets random numbers from the Sonoff dongle's hardware random number generator.
*/
async fn a_main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();

    info!("Opening {DEV_PATH}...");

    let ezsp: EZSP = EZSP::new(DEV_PATH)?;

    for _count in 1../*100*/ {
        let v: u16 = ezsp.getRandomNumber() .await;
        println!("{v:0x}");

        std::thread::sleep(ZZZ);
    };

    Ok(())
}

//---
// Rust has several Futures executor crates, but 'futures-executor' seems recommended.
//      -> https://docs.rs/releases/search?query=futures-executor
//
// See also:
//  - "How to use async/await in Rust when you can't make main function async"
//      -> https://stackoverflow.com/questions/71116502/how-to-use-async-await-in-rust-when-you-cant-make-main-function-async
//  - "How do I call an async function in a match statement under a non-async main function in Rust? [duplicate]"
//      -> https://stackoverflow.com/questions/74623902/how-do-i-call-an-async-function-in-a-match-statement-under-a-non-async-main-func/74624227
//
fn main() -> Result<()> {
    futures_executor::block_on(a_main())
}
