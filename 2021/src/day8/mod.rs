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

struct DisplayAnalysis {
  sample_outputs: Vec<DigitalOutput>,
  final_output: [DigitalOutput; 4],
}

impl DisplayAnalysis {
  fn new(sample_outputs: Vec<DigitalOutput>, final_output: [DigitalOutput; 4]) -> DisplayAnalysis {
    DisplayAnalysis {
      sample_outputs,
      final_output,
    }
  }
}

fn parse_digital_output(raw_output: &str) -> Option<DigitalOutput> {
  let raw_output = raw_output.trim();
  if raw_output.len() > 0 {
    Some(DigitalOutput::new(raw_output.chars().collect()))
  } else {
    None
  }
}

fn parse_display_analysis(raw_line: &str) -> Option<DisplayAnalysis> {
  let line: Vec<&str> = raw_line.split("|").collect();
  if line.len() != 2 {
    return None;
  }
  let sample_outputs: Vec<DigitalOutput> = line[0]
    .split(" ")
    .map(parse_digital_output)
    .filter(|d| d.is_some())
    .map(|d| d.unwrap())
    .collect();
  let mut final_output: Vec<DigitalOutput> = line[1]
    .split(" ")
    .map(parse_digital_output)
    .filter(|d| d.is_some())
    .map(|d| d.unwrap())
    .collect();
  if final_output.len() != 4 {
    return None;
  }

  let o4 = final_output.pop().unwrap();
  let o3 = final_output.pop().unwrap();
  let o2 = final_output.pop().unwrap();
  let o1 = final_output.pop().unwrap();
  Some(DisplayAnalysis::new(sample_outputs, [o1, o2, o3, o4]))
}

fn parse_displays(raw_data: &str) -> Vec<DisplayAnalysis> {
  raw_data
    .split("\n")
    .map(parse_display_analysis)
    .filter(|d| d.is_some())
    .map(|d| d.unwrap())
    .collect()
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

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.data_file)?;
  let displays = parse_displays(&raw_data);
  count_matching_filter(displays, context.config)
}
