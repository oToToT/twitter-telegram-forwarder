use json;
use reqwest;

pub struct Twitter<'a> {
    token: &'a str,
}

impl<'a> Twitter<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }
    pub fn get_user_info(
        &self,
        username: &str,
    ) -> Result<json::JsonValue, Box<dyn std::error::Error>> {
        let endpoint = format!("https://api.twitter.com/2/users/by/username/{}", username);

        let result = self.fetch(&endpoint)?.text()?;
        let result = json::parse(result.as_str())?;
        Ok(result["data"].clone())
    }
    pub fn get_tweets_since(
        &self,
        id: &str,
        since_id: &str,
    ) -> Result<json::JsonValue, Box<dyn std::error::Error>> {
        let endpoint = format!("https://api.twitter.com/2/users/{}/tweets?max_results=100&tweet.fields=created_at&since_id={}", id, since_id);

        let result = self.fetch(&endpoint)?.text()?;
        let result = json::parse(result.as_str())?;
        if result["meta"]["result_count"] == 0 {
            Ok(json::JsonValue::new_array())
        } else {
            Ok(result["data"].clone())
        }
    }
    pub fn get_all_tweets(&self, id: &str) -> Result<json::JsonValue, Box<dyn std::error::Error>> {
        let mut tweets = json::JsonValue::new_array();
        let mut pagination_token: Option<String> = None;
        loop {
            let mut endpoint = format!(
                "https://api.twitter.com/2/users/{}/tweets?max_results=100&tweet.fields=created_at",
                id
            );
            if let Some(pagination_token) = pagination_token {
                endpoint.push_str(format!("&pagination_token={}", pagination_token).as_str());
            }
            let result = self.fetch(&endpoint)?.text()?;
            let result = json::parse(result.as_str())?;

            if result["meta"]["result_count"] == 0 {
                break;
            } else {
                for tweet in result["data"].members() {
                    tweets.push(tweet.clone())?;
                }
                match result["meta"]["next_token"].as_str() {
                    Some(s) => {
                        pagination_token = Some(s.into());
                    }
                    None => {
                        break;
                    }
                };
            }
        }
        Ok(tweets)
    }

    fn fetch(&self, endpoint: &str) -> reqwest::Result<reqwest::blocking::Response> {
        let client = reqwest::blocking::Client::new();
        client
            .get(endpoint)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.token),
            )
            .send()
    }
}
