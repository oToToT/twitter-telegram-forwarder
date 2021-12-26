use ::std::fs;
use urlencoding::encode;
use chrono::DateTime;
use chrono_tz::Asia::Taipei;
use isahc::{prelude::*, Request};
use json;

fn main() -> Result<(), isahc::error::Error> {
    let config_str = fs::read_to_string("config.json").expect("config file not exists");
    let config = json::parse(config_str.as_str()).expect("invalid config format");
    let mut saved_config = config.clone();

    let twitter_token = match config["twitter-token"].as_str() {
        Some(token) => token,
        None => {
            panic!("twitter token not found");
        }
    };
    let twitter_auth = format!("Bearer {}", twitter_token);

    let telegram_token = match config["telegram-token"].as_str() {
        Some(token) => token,
        None => {
            panic!("telegram token not found");
        }
    };
    let telegram_channel_id = match config["telegram-channel"].as_str() {
        Some(id) => id,
        None => {
            panic!("telegram channel id not found");
        }
    };
    let telegram_api = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text=",
        telegram_token, telegram_channel_id
    );

    for (i, account) in config["accounts"].members().enumerate() {
        let name = match account["name"].as_str() {
            Some(name) => name,
            None => {
                panic!("account name not fonud");
            }
        };
        let username = match account["username"].as_str() {
            Some(username) => username,
            None => {
                panic!("account username not fonud");
            }
        };
        let id = match account["id"].as_str() {
            Some(id) => id,
            None => {
                panic!("account id not found");
            }
        };
        let since_id = match account["since_id"].as_str() {
            Some(since_id) => since_id,
            None => {
                panic!("account since_id not found");
            }
        };

        let twitter_api_endpoint = format!("https://api.twitter.com/2/users/{}/tweets?max_results=100&tweet.fields=created_at&since_id={}", id, since_id);
        let mut twitter_resp = Request::get(twitter_api_endpoint)
            .header("Authorization", twitter_auth.clone())
            .body(())?
            .send()?;
        let tweets_result = json::parse(twitter_resp.text()?.as_str()).unwrap();
        if tweets_result["meta"]["result_count"] == 0 {
            continue;
        }

        let mut tweets: Vec<_> = tweets_result["data"].members().collect();
        tweets.sort_by_key(|tweet| {
            match tweet["id"].as_str() {
                Some(tweet_id) => tweet_id.parse::<u64>().unwrap(),
                None => {
                    panic!("invalid tweet response");
                }
            }
        });

        for tweet in tweets {
            let tweet_content = match tweet["text"].as_str() {
                Some(text) => text,
                None => {
                    panic!("invalid tweet response");
                }
            };

            let created_at = match tweet["created_at"].as_str() {
                Some(created_at) => created_at,
                None => {
                    panic!("invalid tweet response");
                }
            };
            let created = DateTime::parse_from_rfc3339(created_at)
                .unwrap()
                .with_timezone(&Taipei)
                .to_rfc2822();

            let tweet_id = match tweet["id"].as_str() {
                Some(tweet_id) => tweet_id,
                None => {
                    panic!("invalid tweet response");
                }
            };
            let url = format!("https://twitter.com/{}/status/{}", username, tweet_id);

            let msg = format!("{}:\n\n{}\n\n{}\n{}", name, tweet_content, created, url);
            let telegram_api_endpoint = format!("{}{}", telegram_api, encode(msg.as_str()));

            let mut tg_resp = isahc::get(telegram_api_endpoint)?;
            let tg_result = json::parse(tg_resp.text()?.as_str()).unwrap();

            match tg_result["ok"].as_bool() {
                Some(ok) => {
                    if !ok {
                        panic!("telegram request failed");
                    }
                },
                None => {
                    panic!("invalid telegram response");
                }
            };

            saved_config["accounts"][i]["since_id"] = tweet_id.into();
            fs::write("config.json", saved_config.dump()).unwrap();
        }
    }

    Ok(())
}
