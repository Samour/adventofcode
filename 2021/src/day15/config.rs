use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
  pub risk_map_file: String,
  pub mult_factor: i32,
  pub strategy: String,
  pub debug: bool,
}
