mod filter;
mod reducer;

use filter::filter_by_mask;
use reducer::{LeastCommonBitReducer, MostCommonBitReducer, ValuesReducer};

use crate::config;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
  data_file: String,
  output: String,
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

fn calculate_power_consumption(data: &Vec<String>) -> Result<(), String> {
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

  Ok(())
}

fn calculate_life_support(data: &Vec<String>) -> Result<(), String> {
  let oxygen_rating_bin =
  filter_by_mask(Box::new(MostCommonBitReducer::new()), &data).ok_or(String::from("Could not match O2 rating"))?;
  let co2_rating_bin =
    filter_by_mask(Box::new(LeastCommonBitReducer::new()), &data).ok_or(String::from("Could not match CO2 rating"))?;

  let oxygen_rating = parse_binary_str(&oxygen_rating_bin);
  let co2_rating = parse_binary_str(&co2_rating_bin);
  println!(
    "oxygen_rating = {}, co2_rating = {}, life_support = {}",
    oxygen_rating,
    co2_rating,
    oxygen_rating * co2_rating
  );

  Ok(())
}

pub fn main(factory: config::ContextFactory) -> Result<(), String> {
  let context: config::Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.data_file)?;
  let parsed_data: Vec<String> = raw_data.split("\n").map(|s| String::from(s)).collect();
  match context.config.output.as_str() {
    "power" => calculate_power_consumption(&parsed_data),
    "life_support" => calculate_life_support(&parsed_data),
    _ => Err(String::from("Unrecognized output type")),
  }
}
