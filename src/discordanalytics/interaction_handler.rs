use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::prelude::*;

pub struct InteractionHandler;

impl EventHandler for InteractionHandler {
  // async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
  //   if let Interaction::Command(command) = interaction {
  //     println!("Received command interaction: {command:#?}");
  //   }
  // }
}