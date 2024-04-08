mod commands;

use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use discordanalytics::all::discordanalytics::DiscordAnalytics;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::gateway::ActivityData;
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler {
  is_loop_running: AtomicBool,
  discordanalytics: Arc<Mutex<DiscordAnalytics>>,
}

#[async_trait]
impl EventHandler for Handler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
      println!("Received command interaction: {:#?}", command.data.name);

      let content = match command.data.name.as_str() {
        "test" => Some(commands::test::run(&command.data.options())),
        _ => Some("not implemented :(".to_string()),
      };

      if let Some(content) = content {
        let data = CreateInteractionResponseMessage::new().content(content);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(why) = command.create_response(&ctx.http, builder).await {
          println!("Cannot respond to slash command: {why}");
        }
      }
    }
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);

    let mut analytics = self.discordanalytics.lock().await;
    analytics.initialize(ready).await;

    let global_command =
      Command::create_global_command(&ctx.http, commands::test::register())
        .await;

    println!("I created the following global slash command: {:?}", global_command.unwrap().name);
  }

  async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
    println!("Cache is ready!");

    let ctx = Arc::new(ctx);

    let analytics = self.discordanalytics.clone();

    if !self.is_loop_running.load(Ordering::Relaxed) {
      tokio::spawn(async move {
        let ctx1 = Arc::clone(&ctx);
        loop {
          let mut analytics = analytics.lock().await;
          analytics.send_data(ctx1.clone()).await;
        }
      });
    }

    self.is_loop_running.swap(true, Ordering::Relaxed);
  }
}

#[tokio::main]
async fn main() {
  dotenv().ok();

  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let mut client = Client::builder(&token, GatewayIntents::all())
    .event_handler(Handler {
      is_loop_running: AtomicBool::new(false),
      discordanalytics: Arc::new(Mutex::new(DiscordAnalytics::new(
        env::var("DISCORD_ANALYTICS_TOKEN").expect("Expected a token in the environment"),
        true,
      ))),
    })
    .activity(ActivityData::playing("with serenity"))
    .await
    .expect("Error creating client");

  if let Err(why) = client.start().await {
    eprintln!("Client error: {why:?}");
  }
}