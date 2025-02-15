use json;
use reqwest;
use urlencoding;

pub struct Telegram<'a> {
    token: &'a str,
}

impl<'a> Telegram<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }
    pub fn send(&self, channel_id: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = format!(
            "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
            self.token,
            channel_id,
            urlencoding::encode(message)
        );

        let result = json::parse(&reqwest::blocking::get(endpoint)?.text()?)?;

        if result["ok"].as_bool().ok_or("invalid telegram response")? {
            Ok(())
        } else {
            Err("telegram request failed".into())
        }
    }
}
