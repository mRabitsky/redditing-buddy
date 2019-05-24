//! Redditing Buddy is a command-line tool that runs in the background, periodically checking various
//! Subreddits for new posts and pinging Discord whenever a new post is detected.
//!
//! USAGE:
//!     redditing-buddy [FLAGS] [OPTIONS]
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! OPTIONS:
//!     -c, --config <config>        Configuration file [default: config.ron]
//!     -d, --duration <duration>    Interstitial duration for checking Reddit [default: 20s]
//! ```

#[macro_use] extern crate structopt;

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use chrono::Local;
use structopt::StructOpt;

use redditing_buddy::config::Config;
use redditing_buddy::Monitor;
use redditing_buddy::utils::{file_exists, parse_duration};

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    /// Configuration file
    #[structopt(short, long, default_value = "config.ron", parse(from_os_str), raw(empty_values = "false", validator_os = "file_exists"))]
    config: PathBuf,

    /// Interstitial duration for checking Reddit
    #[structopt(short, long, default_value = "600s", parse(try_from_str = "parse_duration"))]
    duration: Duration,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let options: Options = Options::from_args();
    let config = Config::read(options.config)?;

    let mut monitor = Monitor::new(config, options.duration);
    monitor.start()?;

    println!("Server started at {}", Local::now());
    println!("Enter \"stop\" to stop the program.");
    loop { // await console input and break when told to stop
        let mut buf = String::new();
        if let Err(e) = io::stdin().read_line(&mut buf) {
            eprintln!("Error: {}", e);
        }

        if buf.trim().to_lowercase() == "stop" { break; }
        else { println!("Sorry mate, didn't catch that!\nIf you want to stop, enter \"stop\" into the console."); }
    }
    println!("Stopping the monitor...");

    monitor.stop()?;

    println!("Server stopped at {}", Local::now());
    Ok(())
}
