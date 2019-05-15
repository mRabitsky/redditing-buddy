//! The Hiring Buddy library is intended to support the CLI app, and doesn't do much as a standalone.
//! It uses `Reqwest` and `Serenity` to query the Reddit API and send updates to Discord, respectively.

#[macro_use] extern crate lazy_static;
extern crate regex;

pub mod config;
pub mod utils;
