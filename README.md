# twitter-telegram-forwarder
A simple twitter -> telegram forwarder written in Rust.

Example channel: https://t.me/nanabunnonijyuuni_tweet

## How to use it?
1. `git clone https://github.com/oToToT/twitter-telegram-forwarder`
2. `cd twitter-telegram-forwarder`
3. `cargo build --release`
4. Setup config.json (see next section)
5. `./target/release/twitter-telegram-forwarder`

## How to setup config.json?
1. Rename `config.json.example` to `config.json`
2. Fill in the twitter [application Bearer token](https://developer.twitter.com/en/docs/authentication/oauth-2-0/application-only)
3. Fill in the telegram [bot token](https://core.telegram.org/bots)
4. Fill in the telegram channel/userid in `"telegram-channel"` field
5. Fill in accounts you want to follow

## Notice
To make it run periodically, you could try setup a [cron](https://en.wikipedia.org/wiki/Cron) job to do so.
