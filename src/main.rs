use chrono::DateTime;
use chrono_tz::Asia::Taipei;
use json;
use std::{thread, time};

mod telegram;
mod twitter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_str = std::fs::read_to_string("config.json").expect("config file not exists");
    let config = json::parse(config_str.as_str()).expect("invalid config format");
    let mut saved_config = config.clone();

    let twitter_token = config["twitter-token"]
        .as_str()
        .expect("twitter token not found");

    let telegram_token = config["telegram-token"]
        .as_str()
        .expect("telegram token not found");
    let telegram_channel_id = config["telegram-channel"]
        .as_str()
        .expect("telegram channel id not found");

    let tw = twitter::Twitter::new(twitter_token);
    let tg = telegram::Telegram::new(telegram_token);

    for (i, account) in config["accounts"].members().enumerate() {
        let username = account["username"]
            .as_str()
            .expect("account username not fonud");
        let name = match account["name"].as_str() {
            Some(s) => s.to_owned(),
            None => {
                let name = tw.get_user_info(&username)?["name"]
                    .as_str()
                    .expect("invalid twitter response")
                    .to_owned();
                saved_config["accounts"][i]["name"] = name.clone().into();
                std::fs::write("config.json", saved_config.dump())?;
                name
            }
        };
        let id = match account["id"].as_str() {
            Some(s) => s.to_owned(),
            None => {
                let id = tw.get_user_info(&username)?["id"]
                    .as_str()
                    .expect("invalid twitter response")
                    .to_owned();
                saved_config["accounts"][i]["id"] = id.clone().into();
                std::fs::write("config.json", saved_config.dump())?;
                id
            }
        };

        let tweets = match account["since_id"].as_str() {
            Some(since_id) => tw.get_tweets_since(&id, since_id)?,
            None => tw.get_all_tweets(&id)?,
        };

        let mut tweets: Vec<_> = tweets.members().collect();
        tweets.sort_by_key(|tweet| {
            tweet["id"]
                .as_str()
                .expect("invalid tweet response")
                .parse::<u64>()
                .expect("invalid tweet id")
        });

        for tweet in tweets {
            let tweet_content = tweet["text"].as_str().expect("invalid tweet response");

            let created_at = tweet["created_at"]
                .as_str()
                .expect("invalid tweet response");
            let created = DateTime::parse_from_rfc3339(created_at)?
                .with_timezone(&Taipei)
                .to_rfc2822();

            let tweet_id = tweet["id"].as_str().expect("invalid tweet response");
            let url = format!("https://twitter.com/{}/status/{}", username, tweet_id);

            let msg = format!("{}:\n\n{}\n\n{}\n{}", name, tweet_content, created, url);
            tg.send(telegram_channel_id, &msg)?;

            saved_config["accounts"][i]["since_id"] = tweet_id.into();
            std::fs::write("config.json", saved_config.dump())?;

            thread::sleep(time::Duration::from_secs(3)); // Follow Telegram's rate limit
        }
    }

    Ok(())
}
