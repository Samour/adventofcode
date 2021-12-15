use serde::Deserialize;
use std::collections::HashMap;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

mod a_star;
mod config;
mod dynamic;

use config::Config;

fn parse_risk_map<'a>(
  raw_map: String,
  config: &Config,
) -> Result<HashMap<(i32, i32), i32>, String> {
  let mut specific_risk: HashMap<(i32, i32), i32> = HashMap::new();
  let mut x: i32 = 0;
  let mut y: i32 = 0;
  for line in raw_map.split("\n") {
    if line.len() == 0 {
      break;
    }
    x = 0;
    for c in line.chars() {
      specific_risk.insert(
        (x, y),
        format!("{}", c)
          .parse()
          .map_err(|e| format!("Failure while parsing risk map"))?,
      );
      x += 1;
    }
    y += 1;
  }

  for i in 0..config.mult_factor {
    for j in 0..config.mult_factor {
      if i == 0 && j == 0 {
        continue;
      }
      for x1 in 0..x {
        for y1 in 0..y {
          specific_risk.insert(
            (i * x + x1, j * y + y1),
            (specific_risk.get(&(x1, y1)).unwrap() + i + j - 1) % 9 + 1,
          );
        }
      }
    }
  }

  Ok(specific_risk)
}

fn find_path(
  risk_map: HashMap<(i32, i32), i32>,
  config: &Config,
  writer: &Writer,
) -> Result<i32, String> {
  let result = match config.strategy.as_str() {
    "A*" => a_star::find_path_cost(risk_map, config, writer)?,
    "dynamic" => dynamic::compute_risk(risk_map)?,
    _ => return Err(format!("Unknown strategy")),
  };
  writer.write(|| format!("Cost of least risky path: {}", result));

  Ok(result)
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<String, String> {
  let context: Context<Config> = factory.create()?;
  let raw_map = context.load_data(&context.config.risk_map_file)?;
  let risk_map = parse_risk_map(raw_map, &context.config)?;
  find_path(risk_map, &context.config, &writer).map(|r| format!("{}", r))
}
