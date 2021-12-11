use serde::Deserialize;
use std::collections::HashMap;

use crate::config::{Context, ContextFactory};

mod digits;

use digits::{DisplayAnalysis, parse_displays};

#[derive(Deserialize)]
struct Config {
  data_file: String,
  mode: String,
  filter_by_segment_count: Option<Vec<usize>>,
  segments_per_digit: Option<HashMap<usize, usize>>,
}

fn count_matching_filter(displays: Vec<DisplayAnalysis>, config: Config) -> Result<(), String> {
  let filter_counts = config.filter_by_segment_count;
  if filter_counts.is_none() {
    return Err(String::from(
      "filter_by_segment_count must be specified for this mode",
    ));
  }
  let filter_counts = filter_counts.unwrap();
  let count = displays
    .iter()
    .flat_map(|d| d.final_output.iter())
    .filter(|d| filter_counts.contains(&d.segments.len()))
    .count();
  println!(
    "Outputs with the required number of enabled segments: {}",
    count
  );

  Ok(())
}

fn select_mode(
  mode: &str,
) -> Result<fn(Vec<DisplayAnalysis>, Config) -> Result<(), String>, String> {
  match mode {
    "count_matching_filter" => Ok(count_matching_filter),
    "derive_outputs" => Err(String::from("TODO Implement")),
    _ => Err(String::from("mode not recognized")),
  }
}

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.data_file)?;
  let displays = parse_displays(&raw_data);
  select_mode(&context.config.mode)?(displays, context.config)
}
