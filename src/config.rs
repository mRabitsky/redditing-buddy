//! All of the configurations for this project are available in this module, including everything
//! relating to API tokens, as well as the actual functional, moving parts of the app.

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use ron;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub subreddit_configs: Vec<SubredditData>,
    pub discord_bot_token: String,
    pub reddit_oauth_id: String,
    pub reddit_oauth_secret: String,
    pub path: PathBuf,
}
impl Config {
    pub fn read(file_path: PathBuf) -> ron::de::Result<Config> {
        let input = fs::read_to_string(file_path)?;
        ron::de::from_str(input.as_str())
    }
    pub fn write(&self, file_path: &PathBuf) -> std::result::Result<(), Box<Error>> {
        fs::write(file_path, ron::ser::to_string_pretty(self, ron::ser::PrettyConfig { ..PrettyConfig::default() })?.as_bytes())?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub before: String, // counterintuitive, but before means things that are NEWER than this
    #[serde(default)]
    pub count: u8,
    #[serde(default)]
    pub limit: u8,
    pub q: String,
    #[serde(default)]
    pub restrict_sr: bool,
    #[serde(default)]
    pub sort: String,
    #[serde(default)]
    pub t: String,
}
impl Default for Query {
    fn default() -> Self {
        Query {
            before: String::with_capacity(9), // two characters for the type code, an underscore, and then the ~6 character name
            count: 0,
            limit: 25,
            q: String::new(),
            restrict_sr: true,
            sort: "new".to_string(),
            t: "all".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SubredditData {
    pub subreddit: String,
    pub search_query: Query,
}
