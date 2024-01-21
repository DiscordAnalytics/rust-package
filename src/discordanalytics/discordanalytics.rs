use reqwest::{Client,header};
use serde_json::json;
use serenity::model::{gateway::Ready, application::Interaction};
use tokio::time::Interval;
use std::time::Duration;

mod api_endpoints {
  pub const BASE_URL: &str = "http://localhost:3001/api";
  pub const EDIT_SETTINGS_URL: &str = "/bots/:id";
  pub const EDIT_STATS_URL: &str = "/bots/:id/stats";
}

mod error_codes {
  pub const INVALID_CLIENT_TYPE: &str = "Invalid client type, please use a valid client.";
  pub const CLIENT_NOT_READY: &str = "Client is not ready, please start the client first.";
  pub const INVALID_RESPONSE: &str = "Invalid response from the API, please try again later.";
  pub const INVALID_API_TOKEN: &str = "Invalid API token, please get one at https://discordanalytics.xyz and try again.";
  pub const DATA_NOT_SENT: &str = "Data cannot be sent to the API, I will try again in a minute.";
  pub const SUSPENDED_BOT: &str = "Your bot has been suspended, please check your mailbox for more information.";
}

pub struct DiscordAnalytics {
  api_token: String,
  headers: header::HeaderMap,
}

impl DiscordAnalytics {
  pub fn new(api_token: String) -> DiscordAnalytics {
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
      api_token,
      headers
    }
  }

  pub async fn init(&self, ready: Ready) {
    let res = Client::new()
      .patch(&format!("{}{}", api_endpoints::BASE_URL, api_endpoints::EDIT_SETTINGS_URL.replace(":id", &ready.user.id.to_string())))
      .headers(self.headers.clone())
      .json(&json!({
        "username": ready.user.name,
        "avatar": ready.user.avatar,
        "framework": "serenity",
        "version": "0.1.0",
      }))
      .send()
      .await
      .unwrap();

    if res.status() == 401 {
      panic!("{}", error_codes::INVALID_API_TOKEN);
    }
    if res.status() == 423 {
      panic!("{}", error_codes::SUSPENDED_BOT);
    }
    if res.status() != 200 {
      panic!("{}", error_codes::INVALID_RESPONSE);
    }
  }

  pub async fn track_interactions(&self, interaction: &Interaction) {
    if let Interaction::Command(command) = interaction {
      println!("Discord Analytics received an interaction: {:#?}", command.data.name);
    }
  }
}