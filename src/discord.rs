use std::collections::HashMap;

use serenity::client::validate_token;
use serenity::http;
use serenity::model::channel::{Message, PrivateChannel};
use serenity::prelude::SerenityError;

use crate::reddit::Post;

pub struct DiscordMessenger(Vec<PrivateChannel>);
impl DiscordMessenger {
    pub fn new(token: String) -> serenity::Result<DiscordMessenger> {
        validate_token(&token)?;

        let token = if token.starts_with("Bot ") {
            token
        } else {
            format!("Bot {}", token)
        };

        http::set_token(&token);
        match DiscordMessenger::_get_dm_channels() {
            Ok(channels) => Ok(DiscordMessenger(channels)),
            Err(e) => Err(e)
        }

    }
    fn _get_dm_channels() -> Result<Vec<PrivateChannel>, serenity::Error> {
        match http::get_current_user()?.guilds()?.first() {
            Some(guild) => guild.id.members::<u64>(None, None)?.iter().map(|member| member.user_id().create_dm_channel()).collect(),
            None => Err(serenity::Error::Other("Bot is not a member of any guild, or the guild couldn't be found!"))
        }
    }

    pub fn send(&self, post: &Post) -> Result<Vec<Message>, SerenityError> {
        self.0.iter().map(|c| c.send_message(|m| m.embed(|em| em.title(&post.title).url(&post.link)))).collect()
    }
    pub fn send_all(&self, posts: HashMap<String, Vec<Post>>) -> Result<Vec<Vec<Message>>, SerenityError> {
        // posts.iter().map(|p| self.send(p)).collect()

        posts.iter().map(|(sub, list)|
            self.0.iter().map(|dm|
                dm.send_message(|m|
                    m.embed(|em|
                        em
                            .title(&sub)
                            .url(format!("https://old.reddit.com/{}", &sub).as_str())
                            .fields(list.iter().map(|p: &Post| (
                                 &p.title,
                                 format!("[{:+}] [{} comment{}] [link]({})\n*posted {} ago*", p.score, &p.comments, if p.comments == 1 { "" } else { "s" }, p.link, humantime::format_duration(p.posted)),
                                 false // inline or not
                            )))
                    )
                )
            ).collect()
        ).collect()
    }
}

#[cfg(test)]
mod discord_tests {
    use std::path::PathBuf;
    use std::time::Duration;
    use url::Url;
    use crate::config::Config;
    use super::*;

    #[test]
    fn send_message() {
        let config = Config::read(PathBuf::from("config.ron")).unwrap();
        let messenger = DiscordMessenger::new(config.discord_bot_token);

        let mut posts = HashMap::new();
        posts.insert(String::from("r/rust"), vec![
            Post {
                title: "Post #1".to_string(),
                link: Url::parse("https://old.reddit.com/r/rust").unwrap(),
                score: 2,
                comments: 1,
                posted: Duration::from_secs(30)
            },
            Post {
                title: "Post #2".to_string(),
                link: Url::parse("https://old.reddit.com/r/rust").unwrap(),
                score: -3,
                comments: 0,
                posted: Duration::from_secs(3600)
            },
            Post {
                title: "Post #3".to_string(),
                link: Url::parse("https://old.reddit.com/r/rust").unwrap(),
                score: 0,
                comments: 15,
                posted: Duration::from_secs(0)
            },
        ]);

        assert!(messenger.unwrap().send_all(posts).is_ok());
    }
}
