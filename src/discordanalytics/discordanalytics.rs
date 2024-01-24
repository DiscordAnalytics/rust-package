use reqwest::{Client,header};
use serenity::{json::{json, JsonMap}, model::{gateway::Ready, application::Interaction}};
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
  data: Vec<Data>,
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

    let data = vec![
      Data {
        date: Utc::now().format("%Y-%m-%d").to_string(),
        guilds: 0,
        users: 0,
        interactions: Vec::new(),
        locales: Vec::new(),
        guildsLocales: Vec::new(),
      }
    ];

    DiscordAnalytics {
      api_token,
      headers,
      data,
    }
  }

  #[tokio::main]
  pub async fn init(&mut self, ready: Ready) {
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

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
    loop {
      interval.tick().await;

      let guilds = ready.guilds.len() as i32;

      // verify if the current data is from today, if not create a new one
      let today_data = self.data.iter_mut().find(|data| data.date == Utc::now().format("%Y-%m-%d").to_string());
      if today_data.is_none() {
        self.data.push(Data {
          date: Utc::now().format("%Y-%m-%d").to_string(),
          guilds,
          users: 0,
          interactions: Vec::new(),
          locales: Vec::new(),
          guildsLocales: Vec::new(),
        });
      }
      let today_data = self.data.iter_mut().find(|data| data.date == Utc::now().format("%Y-%m-%d").to_string()).unwrap();
      println!("Today Data: {:#?}", today_data);
    }
  }

  pub async fn track_interactions(&mut self, interaction: &Interaction) {
    if let Interaction::Command(command) = interaction {
      println!("Discord Analytics received an interaction: {:#?}", command.data.name);

      let user_locale = command.user.locale.clone().unwrap_or("en-US".to_string());
      let guild_locale = command.guild_locale.clone().unwrap_or("en-US".to_string());

      let interaction_name = command.data.name.clone();
      let interaction_type = InteractionType::ApplicationCommand.to_string();

      println!("User locale: {:#?}", user_locale);
      println!("Guild locale: {:#?}", guild_locale);
      println!("Interaction name: {:#?}", interaction_name);
      println!("Interaction type: {:#?}", interaction_type);

      let today_data = self.data.iter_mut().find(|data| data.date == Utc::now().format("%Y-%m-%d").to_string()).unwrap();

      // let mut user_locale_found = false;
      // for locale_data in &mut today_data.locales {
      //   if locale_data.locale == user_locale {
      //     user_locale_found = true;
      //     locale_data.number += 1;
      //   }
      // }
      // if !user_locale_found {
      //   today_data.locales.push(super::data::LocaleData {
      //     locale: user_locale,
      //     number: 1,
      //   });
      // }

      // let mut guild_locale_found = false;
      // for locale_data in &mut today_data.guildsLocales {
      //   if locale_data.locale == guild_locale {
      //     guild_locale_found = true;
      //     locale_data.number += 1;
      //   }
      // }
      // if !guild_locale_found {
      //   today_data.guildsLocales.push(super::data::LocaleData {
      //     locale: guild_locale,
      //     number: 1,
      //   });
      // }

      let mut interaction_found = false;
      for data_interaciton in &mut today_data.interactions {
        if data_interaciton.is_empty() {
          data_interaciton.insert("name".to_string(), json!(interaction_name));
          data_interaciton.insert("number".to_string(), json!(1));
          data_interaciton.insert("type".to_string(), json!(interaction_type));
        }
        if data_interaciton["name"] == interaction_name && data_interaciton["type"] == interaction_type {
          interaction_found = true;
          if let Some(number) = data_interaciton.get_mut("number") {
            if let Some(n) = number.as_i64() {
              *number = json!(n + 1);
            }
          }
        }
      }
      if !interaction_found {
        today_data.interactions.push(JsonMap::from_iter(vec![
          ("name".to_string(), json!(interaction_name)),
          ("number".to_string(), json!(1)),
          ("type".to_string(), json!(interaction_type)),
        ]));
      }

      today_data.users += 1;

      println!("Today Data: {:#?}", today_data);
    }
  }
}