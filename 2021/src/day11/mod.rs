use serde::Deserialize;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

const MAX_LEVEL: i32 = 9;

#[derive(Deserialize)]
struct Config {
  field_file: String,
  mode: String,
  simulate_rounds: Option<i32>,
}

struct Octopus {
  level: i32,
  did_flash: bool,
}

impl Octopus {
  fn new(level: i32) -> Octopus {
    Octopus {
      level,
      did_flash: false,
    }
  }

  fn increment_level(&mut self) -> bool {
    if self.did_flash {
      false
    } else {
      self.level += 1;
      if self.level > MAX_LEVEL {
        self.level = 0;
        self.did_flash = true;
        true
      } else {
        false
      }
    }
  }

  fn complete_tick(&mut self) -> bool {
    let did_flash = self.did_flash;
    self.did_flash = false;
    did_flash
  }
}

struct OctoField {
  octopi: Vec<Vec<Octopus>>,
  flash_count: i32,
}

impl OctoField {
  fn new(octopi: Vec<Vec<Octopus>>) -> OctoField {
    OctoField {
      octopi,
      flash_count: 0,
    }
  }

  fn get(&self, x: usize, y: usize) -> Option<&Octopus> {
    self.octopi.get(y)?.get(x)
  }

  fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Octopus> {
    self.octopi.get_mut(y)?.get_mut(x)
  }

  fn execute_tick(&mut self) -> bool {
    let mut increment_stack: Vec<(usize, usize)> = Vec::new();
    for y in 0..self.octopi.len() {
      for x in 0..self.octopi[y].len() {
        increment_stack.push((x, y));
      }
    }
    let octopi_count = increment_stack.len();

    loop {
      match increment_stack.pop() {
        Some((x, y)) => match self.get_mut(x, y) {
          Some(octopus) => {
            if octopus.increment_level() {
              for neighbour in self.get_neighbours(x, y) {
                increment_stack.push(neighbour);
              }
            }
          }
          None => {}
        },
        None => break,
      }
    }

    let prior_flash_count = self.flash_count;
    for row in &mut self.octopi {
      for octopus in row {
        if octopus.complete_tick() {
          self.flash_count += 1;
        }
      }
    }
    self.flash_count - prior_flash_count == octopi_count as i32
  }

  fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    if x > 0 {
      if y > 0 {
        result.push((x - 1, y - 1));
      }
      result.push((x - 1, y));
      result.push((x - 1, y + 1));
    }
    if y > 0 {
      result.push((x, y - 1));
      result.push((x + 1, y - 1));
    }
    result.push((x, y + 1));
    result.push((x + 1, y));
    result.push((x + 1, y + 1));

    result
      .into_iter()
      .filter(|&(x, y)| self.get(x, y).is_some())
      .collect()
  }
}

fn parse_octo_field(raw_field: String) -> OctoField {
  let octopi: Vec<Vec<Octopus>> = raw_field
    .split("\n")
    .map(|l| l.trim())
    .filter(|l| l.len() > 0)
    .map(|l| {
      l.chars()
        .map(|c| String::from(c).parse::<i32>())
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .map(Octopus::new)
        .collect()
    })
    .collect();

  OctoField::new(octopi)
}

fn count_flashes(mut octo_field: OctoField, config: Config, writer: &Writer) -> Result<i64, String> {
  let simulate_rounds = config
    .simulate_rounds
    .ok_or_else(|| String::from("simulate_rounds is required for this mode"))?;
  for _ in 0..simulate_rounds {
    octo_field.execute_tick();
  }

  writer.write(|| {
    format!(
      "Total number of flashes after {} rounds: {}",
      simulate_rounds, octo_field.flash_count,
    )
  });

  Ok(octo_field.flash_count as i64)
}

fn find_all_flash(mut octo_field: OctoField, writer: &Writer) -> i64 {
  let mut i: i32 = 0;
  loop {
    i += 1;
    if octo_field.execute_tick() {
      break;
    }
  }

  writer.write(|| format!("All flash at step {}", i));
  i as i64
}

fn run_simulation(octo_field: OctoField, config: Config, writer: &Writer) -> Result<i64, String> {
  match config.mode.as_str() {
    "count_flashes" => count_flashes(octo_field, config, writer),
    "find_all_flash" => Ok(find_all_flash(octo_field, writer)),
    _ => Err(String::from("mode not recognized")),
  }
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<i64, String> {
  let context: Context<Config> = factory.create()?;
  let raw_field = context.load_data(&context.config.field_file)?;
  let octo_field = parse_octo_field(raw_field);
  run_simulation(octo_field, context.config, &writer)
}
