use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AppConfig {
  pub title: String,
  pub width: i64,
  pub height: i64,
  pub fullscreen: bool,
  pub mode: String,
  pub target: String,
  pub auto_port: bool,
}

#[derive(Debug, PartialEq, EnumString, Display)]
pub(crate) enum AppConfigMode {
  Process,
  Url
}

impl AppConfig {
  pub(crate) fn default() -> AppConfig {
    AppConfig {
      title: "Dinovel".to_string(),
      width: 800,
      height: 600,
      fullscreen: false,
      mode: AppConfigMode::Url.to_string(),
      target: "https://duckduckgo.com".to_string(),
      auto_port: false,
    }
  }
}
