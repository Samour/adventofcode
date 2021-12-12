use serde::Deserialize;
use std::collections::{HashMap, HashSet};

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

mod digits;
mod solver;

use digits::{parse_displays, DigitalOutput, DisplayAnalysis};
use solver::Solver;

#[derive(Deserialize)]
struct Config {
  data_file: String,
  mode: String,
  filter_by_segment_count: Option<Vec<usize>>,
  canonical_digits: Option<HashMap<usize, String>>,
  disabled_features: Option<Vec<String>>,
}

fn count_matching_filter(
  displays: Vec<DisplayAnalysis>,
  config: Config,
  writer: &Writer,
) -> Result<(), String> {
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
  writer.write(|| {
    format!(
      "Outputs with the required number of enabled segments: {}",
      count
    )
  });

  Ok(())
}

fn derive_outputs(
  displays: Vec<DisplayAnalysis>,
  config: Config,
  writer: &Writer,
) -> Result<(), String> {
  let canonical_digits: HashMap<usize, DigitalOutput> = config
    .canonical_digits
    .ok_or(String::from(
      "canonical_digits must be specified for this mode",
    ))?
    .iter()
    .map(|(&v, chars)| (v, DigitalOutput::new(chars.chars().collect())))
    .collect();
  let disabled_features: HashSet<String> = config
    .disabled_features
    .unwrap_or_else(Vec::new)
    .into_iter()
    .collect();
  let solver = Solver::new(canonical_digits, disabled_features);
  let mut total: i32 = 0;
  for display in displays {
    total += solver.analyze_displays(display)?.extract_value()?;
  }

  writer.write(|| format!("Total of all outputs: {}", total));

  Ok(())
}

fn select_mode(
  mode: &str,
) -> Result<fn(Vec<DisplayAnalysis>, Config, &Writer) -> Result<(), String>, String> {
  match mode {
    "count_matching_filter" => Ok(count_matching_filter),
    "derive_outputs" => Ok(derive_outputs),
    _ => Err(String::from("mode not recognized")),
  }
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.data_file)?;
  let displays = parse_displays(&raw_data);
  select_mode(&context.config.mode)?(displays, context.config, &writer)
}
