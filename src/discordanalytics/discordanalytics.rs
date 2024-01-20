use reqwest::{Client,header};
use serde_json::json;
use serenity::model::gateway::Ready;

mod api_endpoints {
  pub const BASE_URL: &str = "https://discordanalytics.xyz/api";
  pub const EDIT_SETTINGS_URL: &str = "/bots/:id";
  pub const EDIT_STATS_URL: &str = "/bots/:id/stats";
}

mod settings {
  pub const trackInteraction: bool = true;
  pub const trackGuilds: bool = true;
  pub const trackGuildsLocale: bool = true;
  pub const trackUserCount: bool = true;
  pub const trackUserLanguage: bool = true;
}

pub struct DiscordAnalytics {
  client: Ready,
  api_token: String,
  headers: header::HeaderMap,
}

impl DiscordAnalytics {
  pub fn new(client: Ready, api_token: String) -> DiscordAnalytics {
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

  pub async fn track_events(&self) {
    let req_client = Client::new();
    let url = format!("{}{}", api_endpoints::BASE_URL, api_endpoints::EDIT_SETTINGS_URL.replace(":id", &self.client.user.id.to_string()));

    let res = req_client
      .patch(&url)
      .headers(self.headers.clone())
      .json(&json!({
        "username": self.client.user.name,
        "avatar": self.client.user.avatar,
        "framework": "serenity",
        "settings": {
          "trackInteraction": settings::trackInteraction,
          "trackGuilds": settings::trackGuilds,
          "trackGuildsLocale": settings::trackGuildsLocale,
          "trackUserCount": settings::trackUserCount,
          "trackUserLanguage": settings::trackUserLanguage,
        }
      }))
      .send()
      .await
      .unwrap();
    println!("Discord Analytics: {:#?}", res.text().await);
  }
}