# redditing-buddy
Redditing Buddy is a bot that checks for posts matching specific queries in various subreddits of your choosing, and pings you on Discord when it finds something new.

## Usage
Redditing Buddy lets you use a config file to specify what subreddits and queries you want to run, and has command-line options to specify how long it should wait between each check.

An example config file is provided, called `example_config.ron`, which has comments inside detailing how to use each field.

#### Prereqs:
Redditing Buddy expects you to already have:
 
 - a verified Discord bot token (meaning, a token that has already accessed the Discord Gateway at least once)
 - a personal Reddit "script"-type OAuth app (and appropriate secrets)
 
 You'll find a place to plug those tokens at the bottom of the config file.
 
## Bugs or Features?
If you find a bug, let me know! Add an issue or fix it yourself and send a pull-request.

If you find a feature, let me know too! If you want to suggest a feature, consider the lack of a feature the same as a bug (i.e. submit an issue/PR).
