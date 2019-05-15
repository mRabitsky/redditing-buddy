//! Hiring Buddy is a command-line tool that runs in the background, periodically checking various
//! Subreddits for new posts and pinging Discord whenever a new post is detected.
//! ```
//! USAGE:
//!     hiring-buddy [OPTIONS]
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! OPTIONS:
//!     -c, --config <config>        Configuration file [default: config.ron]
//!     -d, --duration <duration>    Interstitial duration for checking Reddit [default: 600s]
//! ```


#[macro_use] extern crate structopt;

use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;

use hiring_buddy::utils::{file_exists, parse_duration};

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

fn main() {
    let options = Options::from_args();
    dbg!(options);
}
