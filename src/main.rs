mod commands;
mod discordanalytics;

use std::env;
use std::sync::Arc;

use discordanalytics::discordanalytics::DiscordAnalytics;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::gateway::ActivityData;
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler {
  discord_analytics: Arc<Mutex<DiscordAnalytics>>,
}

#[async_trait]
impl EventHandler for Handler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    let mut discord_analytics = self.discord_analytics.lock().await;
    discord_analytics.track_interactions(&interaction).await;

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

    let mut discord_analytics = self.discord_analytics.lock().await;
    discord_analytics.init(ready);

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

  let mut client = Client::builder(token, GatewayIntents::all())
    .event_handler(Handler {
      discord_analytics: Arc::new(Mutex::new(DiscordAnalytics::new(env::var("DISCORD_ANALYTICS_TOKEN").expect("Expected a token in the environment"))))
    })
    .activity(ActivityData::playing("with serenity"))
    .await
    .expect("Error creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {why:?}");
  }
}