/*
* We'll need such "pool"
*
* - Pool has a single stream (a literal one, meant!) as its source.
* - Multiple "fishing rods" can be set up on the pool's shore, to filter out certain elements from the one source.
*/
use anyhow::Result;
use log::{info};

use std::time::Duration;
use std::env;
use std::thread;
use std::fmt;

/*
* Print out fishes, forever.
*/
async fn a_main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();

    info!("Creating the Pool");

    let pool: Pool = Pool::new();   // inner mutable

    loop {
        let catch = pool.rx.recv().await ?;

        // Fish as many pikes as you can
        match catch {
            Fish::Hauki | Fish::Ahven =>
                info!("{catch} caught (want more!)"),
            _ =>
                info!("{catch} ignored")
        };
    };
}

fn main() -> Result<()> {
    futures_executor::block_on(a_main())
}

//---
use async_channel::Receiver;
use rand::prelude::*;
use rand::{Rng, rngs::ThreadRng, distributions::{Distribution, Standard, Uniform}};

struct Pool {
    pub rx: Receiver<Fish>
}

impl Pool {
    fn new() -> Self {
        //R let (tx, rx) /*: (Sender<Fish>, Receiver<Fish>)*/ = mpsc::channel();
        let (tx, rx) = async_channel::unbounded();

        /*
        * Forever-running thread; throws some fish to the stream.
        *
        * Rust note: The language is not ready for "async closures" yet, and we don't really need
        *           that here, either. Keeping the thread function normal (never returning), and
        *           using 'futures_executor::block_on' within it works fine.
        */
        thread::spawn(move || /*async*/ {       // Note: neither 'async move ||' or 'move || async' pass
            futures_executor::block_on(async {  // hackish, #boilerplate

                let fish_it /*: impl Iterator<Item = Fish> */ = {
                    let x = StdRng::from_entropy().sample_iter(Standard);
                    x
                };
                //let fish_it = [Fish::Hauki, Fish::Ahven].into_iter();   // TEMP; works

                // Note: We could make another iter (for 'Duration'), or even make one that simultaneously
                //      (with the same 'Rng'!) crates a '(Fish, Duration)'. But here we go another way.
                //
                //      Note 2: Since the above iterator now has ownership of that 'Rng', we need a new
                //          one for the durations.
                //
                let mut rng = rand::thread_rng();

                for fish in fish_it {  // keep getting fish
                    let delay: Duration = Duration::from_millis( rng.gen_range(1000..3000) );

                    thread::sleep(delay);

                    tx.send(fish).await .unwrap();
                }
            });
        });

        Pool{
            rx
        }
    }
}

// From here on, it should be "just basics", but Rust (in 2024) causes pretty much frames to be
// set up, for a small 'enum' that can provide random entries.
//
// For better ways of creating random enums, see
//  - "How do I choose a random value from an enum?" (SO)
//      -> https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum
//
#[derive(Debug)]
pub enum Fish {
    Hauki,
    Ahven,
    Kissankala
}

impl fmt::Display for Fish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*let s = match self {
            Fish::Hauki => "Hauki",
            Fish::Ahven => "Ahven",
            Fish::Kissankala => "Kissankala"
        };*/
        write!(f, "{:?}", self)
    }
}

impl Fish {
    fn from_index(i: u8) -> Fish {
        match i {
            0 => Fish::Hauki,
            1 => Fish::Ahven,
            _ => Fish::Kissankala
        }
    }

    /***R
    fn gen_random<R :Rng>(&mut rng: R) -> Fish {
        match rng.gen_range(0..=2) {
            0 => Fish::Hauki,
            1 => Fish::Ahven,
            _ => Fish::Kissankala
        }
    }
    ***/
}

// From -> https://rust-random.github.io/book/guide-dist.html#uniform-sampling-by-type
//
impl Distribution<Fish> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Fish {
        let i: u8 = rng.gen_range(0..3);
        Fish::from_index(i)
    }
}
