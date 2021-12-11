use serde::Deserialize;
use std::collections::HashMap;

use crate::config::{Context, ContextFactory};

const FISH_RECYCLE: i64 = 6;
const FISH_FIRST_CYCLE: i64 = 8;

#[derive(Deserialize)]
struct Config {
  school_file: String,
  simulate_generations: i32,
}

fn increment_map(map: &mut HashMap<i64, i64>, key: i64, value: i64) {
  let existing = map.remove(&key).unwrap_or(0);
  map.insert(key, existing + value);
}

struct LanternSchool {
  fish: HashMap<i64, i64>,
}

impl LanternSchool {
  fn new(fish: Vec<i64>) -> LanternSchool {
    let mut fish_map: HashMap<i64, i64> = HashMap::new();
    for value in fish {
      increment_map(&mut fish_map, value, 1)
    }

    LanternSchool { fish: fish_map }
  }

  fn increment_day(&mut self) {
    let mut new_fish = HashMap::new();
    for (&days, &count) in &self.fish {
      if days == 0 {
        increment_map(&mut new_fish, FISH_FIRST_CYCLE, count);
        increment_map(&mut new_fish, FISH_RECYCLE, count);
      } else {
        increment_map(&mut new_fish, days - 1, count);
      }
    }

    self.fish = new_fish;
  }
}

fn parse_school(raw_school: String) -> LanternSchool {
  LanternSchool::new(
    raw_school
      .split(",")
      .map(|v| v.parse::<i64>())
      .filter(|v| v.is_ok())
      .map(|v| v.unwrap())
      .collect(),
  )
}

fn execute_simulation(mut school: LanternSchool, generations: i32) {
  for _ in 0..generations {
    school.increment_day();
  }

  let fish_count: i64 = school.fish.values().sum();
  println!("Number of fish: {}", fish_count);
}

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_school = context.load_data(&context.config.school_file)?;
  let school = parse_school(raw_school);
  execute_simulation(school, context.config.simulate_generations);

  Ok(())
}
