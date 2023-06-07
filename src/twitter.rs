use isahc::{prelude::*, Request};
use json;

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

    fn fetch(&self, endpoint: &str) -> Result<isahc::Response<isahc::Body>, isahc::error::Error> {
        Request::get(endpoint)
            .header("Authorization", format!("Bearer {}", self.token))
            .body(())?
            .send()
    }
}
