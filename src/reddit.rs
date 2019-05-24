use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use chrono::Utc;
use reqwest::Client;
use reqwest::header;
use serde::Deserialize;
use url::Url;

use crate::config::Config;
use self::listing::*;

pub struct Post {
    pub title: String,
    pub link: Url,
    pub score: i64,
    pub comments: u64,
    pub posted: Duration,
}

pub struct Redditor {
    client: Client,
    config: Arc<Mutex<Config>>,
    pub token: OAuthToken
}
impl Redditor {
    pub fn new(config: Arc<Mutex<Config>>) -> reqwest::Result<Redditor> {
        let headers: header::HeaderMap = {
            let mut h = header::HeaderMap::new();
            h.insert(header::USER_AGENT, header::HeaderValue::from_str(
                format!("{platform}:{app_id}:{version}",
                        platform="script",
                        app_id=env!("CARGO_PKG_NAME"),
                        version=env!("CARGO_PKG_VERSION")
                ).as_str()
            ).unwrap());
            h
        };
        let client = Client::builder().default_headers(headers).build()?;

        let mut r = Redditor {
            client,
            config,
            token: OAuthToken::new("", 0)
        };
        r._update_token()?;

        Ok(r)
    }

    pub fn check(&mut self) -> HashMap<String, Vec<Post>> {
        if self.token.is_expired() {
            self._update_token().unwrap();
        }
        let mut config = self.config.lock().expect("Arc lock was poisoned in the config");
        let results: HashMap<String, reqwest::Result<reqwest::Response>> = config.subreddit_configs.iter().map(|sub| {
            (sub.subreddit.clone(), self.client
                .get(format!("https://oauth.reddit.com/r/{}/search", sub.subreddit).as_str())
                .bearer_auth(&self.token.token)
                .query(&[
                    ("before", &sub.search_query.before),
                    ("count", &sub.search_query.count.to_string()),
                    ("limit", &sub.search_query.limit.to_string()),
                    ("q", &sub.search_query.q),
                    ("restrict_sr", &sub.search_query.restrict_sr.to_string()),
                    ("sort", &sub.search_query.sort),
                    ("t", &sub.search_query.t),
                ])
                .send())
        }).collect();
        if results.len() == 0 || results.values().any(|v| v.is_err()) {
            return HashMap::new();
        }

        let results: HashMap<String, Thing<Listing>> = results.into_iter().map(|(k, v): (String, reqwest::Result<reqwest::Response>)| (k, v.unwrap().json().unwrap())).collect();

        results.into_iter().map(|(sub, listing_thing): (String, Thing<Listing>)| {
            (format!("r/{}", sub), {
                // first we need to update the subreddit config to be aware of the latest seen post
                let latest = listing_thing.children.first().unwrap().name.clone();
                config.subreddit_configs.iter_mut().find(|c| c.subreddit == sub).unwrap().search_query.before = latest;

                listing_thing.children.iter().map(|post| {
                    Post {
                        title: post.title.clone(),
                        link: post.url.clone(),
                        score: post.score,
                        comments: post.num_comments,
                        posted: Duration::from_secs((Utc::now().timestamp() as u64) - (post.created_utc as u64))
                    }
                }).collect::<Vec<Post>>()
            })
        }).collect()
    }

    fn _update_token(&mut self) -> reqwest::Result<()> {
        let config = self.config.lock().expect("Arc lock was poisoned in the config");
        let resp: AuthResponse = self.client
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(&config.reddit_oauth_id, Some(&config.reddit_oauth_secret))
            .body("grant_type=client_credentials")
            .send()?
            .json()?;

        self.token = OAuthToken::new(resp.access_token, resp.expires_in);
        Ok(())
    }
}
impl Drop for Redditor {
    fn drop(&mut self) {
        let config = self.config.lock().expect("Arc lock was poisoned in the config");
        // manually revoke the token
        self.client
            .post("https://www.reddit.com/api/v1/revoke_token")
            .basic_auth(&config.reddit_oauth_id, Some(&config.reddit_oauth_secret))
            .body(format!("token={}&token_type_hint=access_token", self.token.token))
            .send().expect("Failed to revoke the Reddit token...");
        println!("Successfully revoked the Reddit token, cleaning up the rest now.")
    }
}

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Debug)]
pub struct OAuthToken {
    token: String,
    instant: Instant,
    expiry_duration: Duration,
}
impl OAuthToken {
    fn new(token: impl Into<String>, expires_in: u64) -> OAuthToken {
        OAuthToken {
            token: token.into(),
            instant: Instant::now(),
            expiry_duration: Duration::from_secs(expires_in),
        }
    }
    fn is_expired(&self) -> bool {
        self.instant.elapsed() >= self.expiry_duration
    }
}

mod listing {
    use std::ops::Deref;

    use serde::Deserialize;
    use url::Url;

    #[derive(Debug, Deserialize)]
    pub struct Thing<T> {
        pub data: T,
        pub kind: String,
    }
    impl<T> Deref for Thing<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Listing {
        pub children: Vec<Thing<Link>>,
        pub dist: u8,
    }

    #[derive(Debug, Deserialize)]
    pub struct Link {
        pub created_utc: f64,
        pub name: String,
        pub num_comments: u64,
        pub score: i64,
        pub selftext: String,
        pub subreddit_name_prefixed: String,
        pub title: String,
        #[serde(with = "url_serde")] pub url: Url,
    }
}

#[cfg(test)]
mod reddit_tests {
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    fn auth() {
        let config = Arc::new(Mutex::new(Config::read(PathBuf::from("config.ron")).unwrap()));
        let r = Redditor::new(Arc::clone(&config));

        assert!(r.is_ok());
    }
}
