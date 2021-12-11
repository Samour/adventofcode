use serde::Deserialize;

use crate::config::{Context, ContextFactory};

const FISH_RECYCLE: i32 = 6;
const FISH_FIRST_CYCLE: i32 = 8;

#[derive(Deserialize)]
struct Config {
  school_file: String,
  simulate_generations: i32,
}

struct LanternFish {
  days_until_reproduction: i32,
}

impl LanternFish {
  fn new(days_until_reproduction: i32) -> LanternFish {
    LanternFish {
      days_until_reproduction,
    }
  }

  fn increment_day(&mut self) -> Option<LanternFish> {
    if self.days_until_reproduction == 0 {
      self.days_until_reproduction = FISH_RECYCLE;
      Some(LanternFish::new(FISH_FIRST_CYCLE))
    } else {
      self.days_until_reproduction -= 1;
      None
    }
  }
}

struct LanternSchool {
  fish: Vec<LanternFish>,
}

impl LanternSchool {
  fn new(fish: Vec<LanternFish>) -> LanternSchool {
    LanternSchool { fish }
  }

  fn increment_day(&mut self) {
    let mut new_fish: Vec<LanternFish> = Vec::new();
    for fish in &mut self.fish {
      match fish.increment_day() {
        Some(f) => new_fish.push(f),
        _ => {}
      }
    }
    for fish in new_fish {
      self.fish.push(fish);
    }
  }
}

fn parse_school(raw_school: String) -> LanternSchool {
  LanternSchool::new(
    raw_school
      .split(",")
      .map(|v| v.parse::<i32>())
      .filter(|v| v.is_ok())
      .map(|v| v.unwrap())
      .map(|v| LanternFish::new(v))
      .collect(),
  )
}

fn execute_simulation(mut school: LanternSchool, generations: i32) {
  for _ in 0..generations {
    school.increment_day();
  }

  println!("Number of fish: {}", school.fish.len());
}

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_school = context.load_data(&context.config.school_file)?;
  let school = parse_school(raw_school);
  execute_simulation(school, context.config.simulate_generations);

  Ok(())
}
