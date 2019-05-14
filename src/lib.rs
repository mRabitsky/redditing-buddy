#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::time::Duration;

use regex::Regex;

pub struct Config {
    subreddit_configs: Vec<SubredditData>,
    discord_api_key: String,
}

#[derive(Debug)]
pub struct DurationParseError(&'static str);
impl Display for DurationParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for DurationParseError {}

pub struct SubredditData {
    subreddit: String,
    search_query: String,
    last_seen_post: String,
}

pub fn parse_duration(s: &str) -> Result<Duration, DurationParseError> {
    lazy_static! {
        static ref DURATION_RE: Regex = Regex::new(r"^(\d+)([nμm]?s)$").unwrap();
    }
    let s = s.trim();

    if DURATION_RE.is_match(s) {
        let caps = DURATION_RE.captures(s).unwrap();

        match &caps[2] {
            "ns" => Ok(Duration::from_nanos(caps[1].parse::<u64>().unwrap())),
            "μs" => Ok(Duration::from_micros(caps[1].parse::<u64>().unwrap())),
            "ms" => Ok(Duration::from_millis(caps[1].parse::<u64>().unwrap())),
            "s" => Ok(Duration::from_secs(caps[1].parse::<u64>().unwrap())),
            _ => Err(DurationParseError("Time metric could not be read.")),
        }
    } else {
        Err(DurationParseError(
            "String did not match expected shape of <number><metric>.",
        ))
    }
}

pub fn subreddit_exists(_sub: &str) -> bool {
    return false;
}

#[cfg(test)]
mod parse_tests {
    use std::time::Duration;

    use crate::parse_duration;

    #[test]
    fn parse_failures() {
        let wrong = vec!["ns", "-1s", "s20ms", "20msns"];
        wrong.iter().for_each(|x| {
            assert!(
                parse_duration(x).is_err(),
                format!("Should be error, but got: {}", x)
            )
        });
    }
    #[test]
    fn parse_nano() {
        let a = vec!["2ns", "20ns", "0ns"];
        let b = vec![
            Duration::from_nanos(2),
            Duration::from_nanos(20),
            Duration::from_nanos(0),
        ];

        a.iter()
            .zip(b.iter())
            .for_each(|(x, y)| assert_eq!(parse_duration(x).unwrap(), *y));
    }
    #[test]
    fn parse_micro() {
        let a = vec!["2μs", "20μs", "0μs"];
        let b = vec![
            Duration::from_micros(2),
            Duration::from_micros(20),
            Duration::from_micros(0),
        ];

        a.iter()
            .zip(b.iter())
            .for_each(|(x, y)| assert_eq!(parse_duration(x).unwrap(), *y));
    }
    #[test]
    fn parse_millis() {
        let a = vec!["4ms", "40ms", "0ms"];
        let b = vec![
            Duration::from_millis(4),
            Duration::from_millis(40),
            Duration::from_millis(0),
        ];

        a.iter()
            .zip(b.iter())
            .for_each(|(x, y)| assert_eq!(parse_duration(x).unwrap(), *y));
    }
    #[test]
    fn parse_secs() {
        let a = vec!["5s", "50s", "0s"];
        let b = vec![
            Duration::from_secs(5),
            Duration::from_secs(50),
            Duration::from_secs(0),
        ];

        a.iter()
            .zip(b.iter())
            .for_each(|(x, y)| assert_eq!(parse_duration(x).unwrap(), *y));
    }
}
