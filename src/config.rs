use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum ProfileRegion {
  China,
  Asia,
  Europe,
  America
}

impl ToString for ProfileRegion {
  fn to_string(&self) -> String {
    match self {
      Self::China => "prod_official_cht".to_owned(),
      Self::Asia => "prod_official_asia".to_owned(),
      Self::Europe => "prod_official_eur".to_owned(),
      Self::America => "prod_official_usa".to_owned(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileConfig {
  pub uid: u32,
  pub cookie: String,
  pub power_threshold: u16,
  pub reserve_threshold: u16,
}

impl ProfileConfig {
  pub fn get_region(&self) -> ProfileRegion {
    let n = {
      let mut n = self.uid;
      while n >= 10 {
        n = n / 10;
      }
      n
    };
    match n {
      9 => ProfileRegion::China,
      8 => ProfileRegion::Asia,
      7 => ProfileRegion::Europe,
      6 => ProfileRegion::America,
      _ => panic!("WATAFAK")
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramConfig {
  pub token: String,
  pub chat_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub profiles: Vec<ProfileConfig>,
  pub telegram: Option<TelegramConfig>,
  pub check_in: String,
  pub daily_reminder: String,
  pub weekly_reminder: String,
}