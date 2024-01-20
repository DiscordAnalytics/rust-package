use http::header;
use serenity::Client;
use crate::discordanalytics::interaction_handler;

pub struct DiscordAnalytics {
  client: Client,
  api_token: String,
  headers: http::HeaderMap,
}

impl DiscordAnalytics {
  pub fn new(client: Client, api_token: String) -> DiscordAnalytics {
    let mut headers = header::HeaderMap::new();
    let mut autorization_string = String::from("Bot ");
    autorization_string.push_str(&api_token);
    headers.insert(
      header::AUTHORIZATION,
      header::HeaderValue::from_str(&autorization_string).unwrap(),
    );
    headers.insert(
      header::CONTENT_TYPE,
      header::HeaderValue::from_static("application/json"),
    );

    DiscordAnalytics {
      client,
      api_token,
      headers
    }
  }
}