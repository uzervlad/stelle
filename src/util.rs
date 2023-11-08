use chrono::Local;
use random::Source;

pub fn random_string() -> String {
  let (count, chars) = {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    (chars.len(), chars.chars().collect::<Vec<char>>())
  };

  random::default(Local::now().timestamp_millis().abs() as u64)
    .iter::<usize>()
    .take(6)
    .map(|i| chars[i % count])
    .collect::<String>()
}

pub fn generate_ds() -> String {
  let salt: &'static str = "6s25p5ox5y14umn1p61aqyyvbvvl3lrt";

  let time = Local::now().timestamp();
  let rnd = random_string();
  let hash = md5::compute(format!("salt={}&t={}&r={}", salt, time, rnd));

  format!("{},{},{:x}", time, rnd, hash)
}