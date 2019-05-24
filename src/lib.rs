//! The Redditing Buddy library is intended to support the CLI app, and doesn't do much as a standalone.
//! It uses `Reqwest` and `Serenity` to query the Reddit API and send updates to Discord, respectively.

#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate url;
extern crate url_serde;

pub mod config;
pub mod discord;
pub mod monitor;
pub mod reddit;
pub mod utils;

pub use monitor::Monitor;
