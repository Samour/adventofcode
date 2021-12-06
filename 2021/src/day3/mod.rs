mod reducer;

use reducer::{LeastCommonBitReducer, MostCommonBitReducer, ValuesReducer};

use crate::config;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
  data_file: String,
}

fn parse_binary_str(binary: &str) -> i32 {
  let mut result: i32 = 0;
  for c in binary.chars() {
    result *= 2;
    if c == '1' {
      result += 1;
    }
  }

  result
}

fn calculate_power_consumption(data: &Vec<String>) {
  let gamma_bin = MostCommonBitReducer::new().reduce(data);
  let epsilon_bin = LeastCommonBitReducer::new().reduce(data);
  let gamma = parse_binary_str(&gamma_bin);
  let epsilon = parse_binary_str(&epsilon_bin);

  println!(
    "gamma = {}, epsilon = {}, power = {}",
    gamma,
    epsilon,
    gamma * epsilon
  );
}

pub fn main(factory: config::ContextFactory) -> Result<(), String> {
  let context: config::Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.data_file)?;
  let parsed_data: Vec<String> = raw_data.split("\n").map(|s| String::from(s)).collect();
  calculate_power_consumption(&parsed_data);

  Ok(())
}
