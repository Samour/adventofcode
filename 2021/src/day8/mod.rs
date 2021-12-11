use serde::Deserialize;
use std::collections::HashSet;

use crate::config::{Context, ContextFactory};

#[derive(Deserialize)]
struct Config {
  data_file: String,
  filter_by_segment_count: Option<Vec<usize>>,
}

struct DigitalOutput {
  segments: HashSet<char>,
}

impl DigitalOutput {
  fn new(segments: HashSet<char>) -> DigitalOutput {
    DigitalOutput { segments }
  }
}

fn parse_digital_output(raw_output: String) -> Option<DigitalOutput> {
  let raw_output = raw_output.trim();
  if raw_output.len() > 0 {
    Some(DigitalOutput::new(raw_output.chars().collect()))
  } else {
    None
  }
}

fn parse_all_output_digits(raw_data: String) -> Vec<DigitalOutput> {
  raw_data
    .split("\n")
    .map(|l| l.split("|").collect::<Vec<&str>>())
    .filter(|l| l.len() > 1)
    .map(|l| l[1])
    .flat_map(|o| o.split(" "))
    .map(String::from)
    .map(parse_digital_output)
    .filter(|d| d.is_some())
    .map(|d| d.unwrap())
    .collect()
}

fn count_matching_filter(digits: Vec<DigitalOutput>, config: Config) -> Result<(), String> {
  let filter_counts = config.filter_by_segment_count;
  if filter_counts.is_none() {
    return Err(String::from(
      "filter_by_segment_count must be specified for this mode",
    ));
  }
  let filter_counts = filter_counts.unwrap();
  let count = digits
    .iter()
    .filter(|d| filter_counts.contains(&d.segments.len()))
    .count();
  println!(
    "Outputs with the required number of enabled segments: {}",
    count
  );

  Ok(())
}

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.data_file)?;
  let digits = parse_all_output_digits(raw_data);
  count_matching_filter(digits, context.config)
}
