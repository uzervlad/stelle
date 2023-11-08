use std::collections::HashMap;

use eyre::Result;
use serde::{Serialize, Deserialize};

use crate::config::TelegramConfig;

pub struct TelegramBot {
  token: String,
  chat_id: u64,
  client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelegramResponse {
  pub ok: bool,
  pub description: Option<String>,
}

impl TelegramBot {
  pub fn create(cfg: TelegramConfig) -> Self {
    Self {
      token: cfg.token,
      chat_id: cfg.chat_id,
      client: reqwest::Client::new(),
    }
  }

  pub async fn send_message(&self, message: String) -> Result<TelegramResponse> {
    let params = HashMap::from([
      ("text", message),
      ("chat_id", self.chat_id.to_string()),
    ]);

    let response = self.client
      .post(format!("https://api.telegram.org/bot{}/sendMessage", self.token))
      .json(&params)
      .send().await?
      .json().await?;

    Ok(response)
  }
}