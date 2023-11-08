use std::collections::HashMap;

use reqwest::header::HeaderMap;
use eyre::Result;
use serde::{Serialize, Deserialize};

use crate::{config::ProfileConfig, util::generate_ds};

#[derive(Serialize, Deserialize)]
pub struct HoyoResponse<T> {
  pub retcode: u32,
  pub message: String,
  pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct GameExpedition {
  pub status: String,
  pub remaining_time: u32,
  pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GameRecords {
  pub current_stamina: u16,
  pub max_stamina: u16,
  pub accepted_epedition_num: u8,
  pub total_expedition_num: u8,
  pub current_train_score: u16,
  pub max_train_score: u16,
  pub current_rogue_score: u16,
  pub max_rogue_score: u16,
  pub current_reserve_stamina: u16,
  pub is_reserve_stamina_full: bool,
  pub expeditions: Vec<GameExpedition>,
}

#[derive(Serialize, Deserialize)]
pub struct CheckInData {
  pub today_sign_day: u8,
  pub today: String,
  pub is_sign: bool,
  pub is_sub: bool,
  pub sign_cnt_missed: u8,
  // pub short_sign_day: u8,
}

#[derive(Serialize, Deserialize)]
pub struct CheckInAward {
  pub icon: String,
  pub name: String,
  pub cnt: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CheckInAwards {
  pub month: u8,
  pub awards: Vec<CheckInAward>,
  // pub biz: String,
  pub resign: bool,
  // short_extra_reward - Limited Check-In Event
}

impl ProfileConfig {
  pub async fn get_game_records(&self) -> Result<HoyoResponse<GameRecords>> {
    let region = self.get_region();
    let client = reqwest::Client::new();
    let query = &[("server", region.to_string()), ("role_id", self.uid.to_string())];
    
    let mut headers = HeaderMap::new();
    headers.insert("x-rpc-app_version", "1.5.0".parse().unwrap());
    headers.insert("x-rpc-client_type", 5.into());
    headers.insert("x-rpc-language", "en-us".parse().unwrap());
    headers.insert("Cookie", self.cookie.parse().unwrap());
    headers.insert("DS", generate_ds().parse().unwrap());

    let response = client
      .get("https://bbs-api-os.hoyolab.com/game_record/hkrpg/api/note")
      .query(query)
      .headers(headers)
      .send().await?
      .json().await?;

    Ok(response)
  }

  pub async fn get_checkin_status(&self) -> Result<HoyoResponse<CheckInData>> {
    let client = reqwest::Client::new();
    let query = &[("act_id", "e202303301540311")];

    let mut headers = HeaderMap::new();
    headers.insert("Cookie", self.cookie.parse().unwrap());

    let response = client
      .get("https://sg-public-api.hoyolab.com/event/luna/os/info")
      .query(query)
      .headers(headers)
      .send().await?
      .json().await?;

    Ok(response)
  }

  pub async fn get_checkin_awards(&self) -> Result<HoyoResponse<CheckInAwards>> {
    let client = reqwest::Client::new();
    let query = &[("act_id", "e202303301540311")];

    let mut headers = HeaderMap::new();
    headers.insert("Cookie", self.cookie.parse().unwrap());

    let response = client
      .get("https://sg-public-api.hoyolab.com/event/luna/os/home")
      .query(query)
      .headers(headers)
      .send().await?
      .json().await?;

    Ok(response)
  }

  pub async fn collect_checkin(&self) -> Result<()> {
    let client = reqwest::Client::new();
    
    let mut headers = HeaderMap::new();
    headers.insert("Cookie", self.cookie.parse().unwrap());

    let response = client
      .post("https://sg-public-api.hoyolab.com/event/luna/os/sign")
      .json(&HashMap::from([("act_id", "e202303301540311")]))
      .headers(headers)
      .send().await?
      .text().await?;

    Ok(())
  }
}