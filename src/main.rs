use std::{fs, path::Path};

use config::Config;
use eyre::Result;
use scheduler::Scheduler;
use telegram::TelegramBot;

mod config;
mod scheduler;
mod tasks;
mod telegram;
mod profile;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
  let config: Config = {
    let config = fs::read(Path::new("./config.json")).expect("config file");
    serde_json::from_slice(config.as_slice()).expect("config parsing")
  };
  let mut scheduler = Scheduler::new(&config);
  let tg = config.telegram.map(|cfg| TelegramBot::create(cfg));

  Ok(())
}
