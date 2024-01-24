use std::fmt;

use serenity::json::JsonMap;

#[derive(Debug, PartialEq)]
pub enum InteractionType {
  Unknown,
  Ping,
  ApplicationCommand,
  MessageComponent,
  ApplicationCommandAutocomplete,
  ModalSubmit,
}

impl fmt::Display for InteractionType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      InteractionType::Unknown => write!(f, "Unknown"),
      InteractionType::Ping => write!(f, "Ping"),
      InteractionType::ApplicationCommand => write!(f, "ApplicationCommand"),
      InteractionType::MessageComponent => write!(f, "MessageComponent"),
      InteractionType::ApplicationCommandAutocomplete => write!(f, "ApplicationCommandAutocomplete"),
      InteractionType::ModalSubmit => write!(f, "ModalSubmit"),
    }
  }
}

pub type Locale = String;

#[derive(Debug)]
pub struct Data {
  pub date: String,
  pub guilds: i32,
  pub users: i32,
  pub interactions: Vec<JsonMap>,
  pub locales: Vec<LocaleData>,
  pub guildsLocales: Vec<LocaleData>,
}

#[derive(Debug)]
pub struct LocaleData {
  pub locale: Locale,
  pub number: i32,
}
