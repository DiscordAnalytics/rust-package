mod commands;
mod discordanalytics;

use std::env;

use discordanalytics::discordanalytics::DiscordAnalytics;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
      println!("Received command interaction: {command:#?}");

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

    let discord_analytics = DiscordAnalytics::new(ready, env::var("DISCORD_ANALYTICS_TOKEN").expect("Expected a token in the environment"));
    discord_analytics.track_events().await;

    let global_command =
      Command::create_global_command(&ctx.http, commands::test::register())
        .await;

    println!("I created the following global slash command: {:?}", global_command.unwrap().name);
  }
}

#[tokio::main]
async fn main() {
  dotenv().ok();

  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let mut client = Client::builder(token, GatewayIntents::empty())
    .event_handler(Handler)
    .await
    .expect("Error creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {why:?}");
  }
}