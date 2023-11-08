use async_trait::async_trait;
use chrono::{DateTime, Local};
use eyre::Result;

use crate::{tasks::{check_in::CheckInTask, expedition::ExpeditionTask}, config::{Config, ProfileConfig}, telegram::TelegramBot};

#[async_trait]
pub trait SchedulerTask {
  fn get_next_run(&self) -> Option<DateTime<Local>>;

  async fn run_task(&mut self, tg: &Option<TelegramBot>, profile: &ProfileConfig) -> Result<()>;

  fn get_name(&self) -> &String;
}

pub struct Scheduler {
  tasks: Vec<Box<dyn SchedulerTask>>,
}

impl Scheduler {
  pub fn new(config: &Config) -> Self {
    let mut tasks: Vec<Box<dyn SchedulerTask>> = Vec::new();
    tasks.push(Box::new(CheckInTask::new(&config.check_in)));
    tasks.push(Box::new(ExpeditionTask::new()));

    Self {
      tasks
    }
  }

  pub fn check(&mut self) {
    // for task in self.tasks.iter_mut() {
    //   // task.get_next_run()
    // }
  }
}