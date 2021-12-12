use serde::Deserialize;
use std::string::String;
use std::vec::Vec;

use crate::config;
use crate::writer::Writer;

#[derive(Deserialize)]
struct Config {
  mode: String,
  window_size: Option<usize>,
  measurements_file: String,
}

fn parse_measurements(raw_content: String) -> Vec<i32> {
  raw_content
    .split("\n")
    .map(|i| i.parse())
    .filter(|i| i.is_ok())
    .map(|i| i.unwrap())
    .collect()
}

fn count_increases(measurements: Vec<i32>, writer: Writer) -> Result<(), String> {
  let mut inc_count = 0;
  for i in 1..measurements.len() {
    if measurements[i] > measurements[i - 1] {
      inc_count += 1;
    }
  }

  writer.write(|| format!("Total number of increases: {}", inc_count));

  Ok(())
}

fn create_windows(window_size: usize, measurements: Vec<i32>) -> Vec<i32> {
  let mut result: Vec<i32> = Vec::new();
  for i in window_size..measurements.len() + 1 {
    let mut window_value: i32 = 0;
    for j in (i - window_size)..i {
      window_value += measurements[j];
    }
    result.push(window_value);
  }

  result
}

fn count_windowed_increases(
  window_size: usize,
  measurements: Vec<i32>,
  writer: Writer,
) -> Result<(), String> {
  let windowed = create_windows(window_size, measurements);

  count_increases(windowed, writer)
}

pub fn main(factory: config::ContextFactory, writer: Writer) -> Result<(), String> {
  let context: config::Context<Config> = factory.create()?;
  let raw_content = context.load_data(&context.config.measurements_file)?;
  let measurements = parse_measurements(raw_content);

  if context.config.mode == "count_increases" {
    count_increases(measurements, writer)
  } else if context.config.mode == "count_windowed_increases" {
    match context.config.window_size {
      Some(ws) => count_windowed_increases(ws, measurements, writer),
      None => Err(String::from("parameter window_size missing from config")),
    }
  } else {
    Err(format!("Unrecognized mode: {}", &context.config.mode))
  }
}
