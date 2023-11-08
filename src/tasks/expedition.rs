use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Local};
use cron::Schedule;
use eyre::Result;

use crate::{scheduler::SchedulerTask, telegram::TelegramBot, config::ProfileConfig};

pub struct ExpeditionTask {
  name: String,
  schedule: Schedule,
  last_all_done: bool,
}

impl ExpeditionTask {
  pub fn new() -> Self {
    Self {
      name: "expedition".to_owned(),
      schedule: Schedule::from_str("0 */30 * * * *").unwrap(),
      last_all_done: false,
    }
  }
}

#[async_trait]
impl SchedulerTask for ExpeditionTask {
  fn get_next_run(&self) -> Option<DateTime<Local>> {
    self.schedule.upcoming(Local).next()
  }

  async fn run_task(&mut self, tg: &Option<TelegramBot>, profile: &ProfileConfig) -> Result<()> {
    let records = profile.get_game_records().await?;
    let all_done = records.data.expeditions.iter().all(|e| e.remaining_time == 0);
    if all_done && !self.last_all_done {
      if let Some(tg) = tg {
        tg.send_message(format!("[{}] NAME_PLACEHOLDER: All assignments are finished!", profile.uid)).await?;
      }
    }
    self.last_all_done = all_done;
    todo!()
  }

  fn get_name(&self) -> &String {
    &self.name
  }
}