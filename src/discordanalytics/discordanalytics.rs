use reqwest::{Client,header};
use serde_json::json;
use serenity::model::{gateway::Ready, application::Interaction};
// use tokio::time::Interval;
// use std::time::Duration;
// use std::collections::HashMap;
use chrono::Utc;
use crate::discordanalytics::data::InteractionType;

use super::data::Data;

mod api_endpoints {
  pub const BASE_URL: &str = "http://localhost:3001/api";
  pub const EDIT_SETTINGS_URL: &str = "/bots/:id";
  pub const EDIT_STATS_URL: &str = "/bots/:id/stats";
}

mod error_codes {
  pub const INVALID_RESPONSE: &str = "Invalid response from the API, please try again later.";
  pub const INVALID_API_TOKEN: &str = "Invalid API token, please get one at https://discordanalytics.xyz and try again.";
  pub const DATA_NOT_SENT: &str = "Data cannot be sent to the API, I will try again in a minute.";
  pub const SUSPENDED_BOT: &str = "Your bot has been suspended, please check your mailbox for more information.";
}

pub struct DiscordAnalytics {
  api_token: String,
  headers: header::HeaderMap,
  data: Data
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

    let data = Data {
      date: Utc::now().format("%Y-%m-%d").to_string(),
      guilds: 0,
      users: 0,
      interactions: Vec::new(),
      locales: Vec::new(),
      guildsLocales: Vec::new(),
    };

    DiscordAnalytics {
      api_token,
      headers,
      data
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

      let user_locale = command.user.locale.clone();
      let guild_locale = command.guild_locale.clone();

      let interaction_name = command.data.name.clone();
      let interaction_type = InteractionType::ApplicationCommand;

      println!("User locale: {:#?}", user_locale);
      println!("Guild locale: {:#?}", guild_locale);
      println!("Interaction name: {:#?}", interaction_name);
      println!("Interaction type: {:#?}", interaction_type);
    }
  }
}