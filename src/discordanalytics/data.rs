
#[derive(Debug)]
pub enum InteractionType {
  Unknown,
  Ping,
  ApplicationCommand,
  MessageComponent,
  ApplicationCommandAutocomplete,
  ModalSubmit,
}

pub type Locale = String;

pub struct Data {
  pub date: String,
  pub guilds: i32,
  pub users: i32,
  pub interactions: Vec<Interaction>,
  pub locales: Vec<LocaleData>,
  pub guildsLocales: Vec<LocaleData>,
}

pub struct Interaction {
  pub name: String,
  pub number: i32,
  pub type_: InteractionType,
}

pub struct LocaleData {
  pub locale: Locale,
  pub number: i32,
}
