use std::{sync::Arc, time::Duration};

use reqwest::{Client,header};
use serenity::{client::Context, json::json, model::gateway::Ready};
use chrono::Utc;

use crate::discordanalytics::data::{Data, GuildMembersData};

mod api_endpoints {
  pub const BASE_URL: &str = "https://discordanalytics.xyz/api";
  pub const BOT_URL: &str = "/bots/:id";
  pub const STATS_URL: &str = "/bots/:id/stats";
}

mod error_codes {
  pub const INVALID_CLIENT_TYPE: &str = "Invalid client type, please use a valid client.";
  pub const CLIENT_NOT_READY: &str = "Client is not ready, please start the client first.";
  pub const INVALID_RESPONSE: &str = "Invalid response from the API, please try again later.";
  pub const INVALID_API_TOKEN: &str = "Invalid API token, please get one at https://discordanalytics.xyz and try again.";
  pub const DATA_NOT_SENT: &str = "Data cannot be sent to the API, I will try again in a minute.";
  pub const SUSPENDED_BOT: &str = "Your bot has been suspended, please check your mailbox for more information.";
  pub const INSTANCE_NOT_INITIALIZED: &str = "It seem that you didn't initialize your instance. Please check the docs for more informations.";
}

pub struct DiscordAnalytics {
  api_key: String,
  debug: bool,
  is_ready: bool,
  headers: header::HeaderMap,
  data: Data,
}

impl DiscordAnalytics {
  pub fn new(api_key: String, debug: bool) -> Self {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", format!("Bot {}", api_key).parse().unwrap());

    DiscordAnalytics {
      api_key,
      debug,
      is_ready: false,
      headers,
      data: Data {
        date: Utc::now().format("%Y-%m-%d").to_string(),
        guilds: 0,
        users: 0,
        interactions: Vec::new(),
        locales: Vec::new(),
        guilds_locales: Vec::new(),
        guild_members: GuildMembersData {
          little: 0,
          medium: 0,
          big: 0,
          huge: 0,
        },
      },
    }
  }

  pub async fn initialize(&mut self, ready: Ready) {
    let response = Client::new()
      .patch(
        format!(
          "{}{}",
          api_endpoints::BASE_URL, api_endpoints::BOT_URL.replace(":id", &ready.user.id.to_string())
        )
      )
      .headers(self.headers.clone())
      .json(&json!({
        "username": ready.user.name,
        "avatar": ready.user.avatar,
        "framework": "serenity",
        "version": env!("CARGO_PKG_VERSION"),
      }))
      .send()
      .await
      .unwrap();

    if response.status() == 401 {
      panic!("{}", error_codes::INVALID_API_TOKEN);
    }
    if response.status() == 423 {
      panic!("{}", error_codes::SUSPENDED_BOT);
    }
    if response.status() != 200 {
      panic!("{}", error_codes::INVALID_RESPONSE);
    }

    if self.debug {
      println!("[DISCORDANALYTICS] Instance successfully initialized");
    }
    self.is_ready = true;

    if self.debug {
      // check if --dev is in launch arguments
      if std::env::args().any(|arg| arg == "--dev") {
        println!("[DISCORDANALYTICS] DevMode is enabled. Stats will be sent every 30s.");
      } else {
        println!("[DISCORDANALYTICS] DevMode is disabled. Stats will be sent every 5 minutes.");
      }
    }
  }

  pub async fn send_data(&mut self, ctx: Arc<Context>) {
    if !self.is_ready {
      panic!("{}", error_codes::INSTANCE_NOT_INITIALIZED);
    }

    if self.debug {
      println!("[DISCORDANALYTICS] Sending stats...");
    }

    let guild_count = ctx.cache.guild_count() as i32;
    let user_count = ctx.cache.user_count() as i32;

    let response = Client::new()
      .post(
        format!(
          "{}{}",
          api_endpoints::BASE_URL, api_endpoints::STATS_URL.replace(":id", &ctx.cache.current_user().id.to_string())
        )
      )
      .headers(self.headers.clone())
      .json(&json!({
        "date": Utc::now().format("%Y-%m-%d").to_string(),
        "guilds": guild_count,
        "users": user_count,
        "interactions": json!(self.data.interactions),
        "locales": json!(self.data.locales),
        "guildsLocales": json!(self.data.guilds_locales),
        "guildMembers": json!(self.data.guild_members),
      }))
      .send()
      .await
      .unwrap();

    if response.status() == 401 {
      panic!("{}", error_codes::INVALID_API_TOKEN);
    }
    if response.status() == 423 {
      panic!("{}", error_codes::SUSPENDED_BOT);
    }
    if response.status() != 200 {
      panic!("{} {}", error_codes::INVALID_RESPONSE, response.text().await.unwrap());
    }
    if response.status().is_success() {
      if self.debug {
        println!("[DISCORDANALYTICS] Stats {} sent to the API", self.data);
      }

      self.data = Data {
        date: Utc::now().format("%Y-%m-%d").to_string(),
        guilds: guild_count,
        users: user_count,
        interactions: Vec::new(),
        locales: Vec::new(),
        guilds_locales: Vec::new(),
        guild_members: self.calculate_guild_members_repartition(ctx),
      };
    }

    tokio::time::sleep(Duration::from_secs(if std::env::args().any(|arg| arg == "--dev") { 30 } else { 300 })).await;
  }

  pub fn calculate_guild_members_repartition(&mut self, ctx: Arc<Context>) -> GuildMembersData {
    let mut little = 0;
    let mut medium = 0;
    let mut big = 0;
    let mut huge = 0;

    for guild in ctx.cache.guilds() {
      let guild = ctx.cache.guild(guild).unwrap();
      let member_count = guild.member_count as i32;

      if member_count <= 100 {
        little += 1;
      } else if member_count <= 500 {
        medium += 1;
      } else if member_count <= 1500 {
        big += 1;
      } else {
        huge += 1;
      }
    }

    return GuildMembersData {
      little,
      medium,
      big,
      huge,
    };
  }
}