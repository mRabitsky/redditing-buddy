#[macro_use]
extern crate structopt;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::time::Duration;

use structopt::StructOpt;

use hiring_buddy::parse_duration;


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

fn file_exists(file: &OsStr) -> Result<(), OsString> {
    let path = Path::new(&file);
    if path.exists() {
        if path.extension().is_none() {
            Err(OsString::from("Could not read the file extension"))
        } else if path.extension().unwrap() != "ron" {
            Err(OsString::from("Config files must be RON files (Rusty Object Notation)"))
        } else {
            Ok(())
        }
    } else {
        Err(OsString::from("There is no file at the path entered"))
    }
}

fn main() {
    let options = Options::from_args();
    dbg!(options);
}
