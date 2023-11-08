use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Local};
use cron::Schedule;
use eyre::Result;

use crate::{scheduler::SchedulerTask, telegram::TelegramBot, config::ProfileConfig};

pub struct CheckInTask {
  name: String,
  schedule: Schedule,
}

impl CheckInTask {
  pub fn new(schedule: &String) -> Self {
    Self {
      name: "check_in".to_owned(),
      schedule: Schedule::from_str(&schedule).unwrap(),
    }
  }
}

#[async_trait]
impl SchedulerTask for CheckInTask {
  fn get_next_run(&self) -> Option<DateTime<Local>> {
    self.schedule.upcoming(Local).next()
  }

  async fn run_task(&mut self, tg: &Option<TelegramBot>, profile: &ProfileConfig) -> Result<()> {
    // TODO
    Ok(())
  }

  fn get_name(&self) -> &String {
    &self.name
  }
}