use std::fmt;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum InteractionType {
  Ping = 1,
  ApplicationCommand = 2,
  MessageComponent = 3,
  ApplicationCommandAutocomplete = 4,
  ModalSubmit = 5,
}

impl fmt::Display for InteractionType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      InteractionType::Ping => write!(f, "Ping"),
      InteractionType::ApplicationCommand => write!(f, "ApplicationCommand"),
      InteractionType::MessageComponent => write!(f, "MessageComponent"),
      InteractionType::ApplicationCommandAutocomplete => write!(f, "ApplicationCommandAutocomplete"),
      InteractionType::ModalSubmit => write!(f, "ModalSubmit"),
    }
  }
}

pub type Locale = String;

#[derive(Debug, Serialize)]
pub struct Data {
  pub date: String,
  pub guilds: i32,
  pub users: i32,
  pub interactions: Vec<InteractionData>,
  pub locales: Vec<LocaleData>,
  pub guilds_locales: Vec<LocaleData>,
  pub guild_members: GuildMembersData,
}

impl fmt::Display for Data {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Data {{ date: {}, guilds: {}, users: {}, interactions: {:?}, locales: {:?}, guilds_locales: {:?}, guild_members: {:?} }}",
      self.date, self.guilds, self.users, self.interactions, self.locales, self.guilds_locales, self.guild_members
    )
  }
}

#[derive(Debug, Serialize)]
pub struct InteractionData {
  pub name: String,
  pub number: i32,
  #[serde(rename = "type")]
  pub interaction_type: InteractionType,
}

#[derive(Debug, Serialize)]
pub struct LocaleData {
  pub locale: Locale,
  pub number: i32,
}

#[derive(Debug, Serialize)]
pub struct GuildMembersData {
  pub little: i32,
  pub medium: i32,
  pub big: i32,
  pub huge: i32,
}
