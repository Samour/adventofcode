use serde::Deserialize;

use crate::config::{Context, ContextFactory};

const LOOP_ESCAPE: i32 = 10_000;

#[derive(Deserialize)]
struct Config {
  trace_search: Option<bool>,
  data_file: String,
  search_method: String,
}

struct Crab {
  original_position: i32,
}

impl Crab {
  fn new(original_position: i32) -> Crab {
    Crab { original_position }
  }
}

fn compute_cost(population: &Vec<&Crab>, position: i32) -> i32 {
  let mut cost = 0;
  for crab in population {
    cost += (crab.original_position - position).abs();
  }

  cost
}

fn print_position(trace_search: bool, position: i32, cost: i32) {
  if trace_search {
    println!("Position = {}, Cost = {}", position, cost);
  }
}

struct SearchOutcome {
  position: i32,
  expended_fuel: i32,
}

struct HillClimbAlg<'a> {
  trace_climb: bool,
  population: Vec<&'a Crab>,
}

impl HillClimbAlg<'_> {
  fn new<'a>(population: Vec<&'a Crab>, trace_climb: bool) -> HillClimbAlg<'a> {
    HillClimbAlg {
      population,
      trace_climb,
    }
  }

  fn seed_position(&self) -> i32 {
    let position_sum: i32 = self.population.iter().map(|c| c.original_position).sum();

    position_sum / self.population.len() as i32
  }

  fn compute_cost(&self, position: i32) -> i32 {
    compute_cost(&self.population, position)
  }

  fn choose_direction(&self, seed_pos: i32) -> SearchOutcome {
    let cost_seed = self.compute_cost(seed_pos);
    let cost_left = self.compute_cost(seed_pos - 1);
    let cost_right = self.compute_cost(seed_pos + 1);
    if cost_left > cost_seed && cost_right > cost_seed {
      SearchOutcome {
        position: 0,
        expended_fuel: cost_seed,
      }
    } else if cost_left < cost_right {
      SearchOutcome {
        position: -1,
        expended_fuel: cost_left,
      }
    } else {
      SearchOutcome {
        position: 1,
        expended_fuel: cost_right,
      }
    }
  }

  fn print_position(&self, position: i32, cost: i32) {
    print_position(self.trace_climb, position, cost);
  }

  fn execute(&self) -> SearchOutcome {
    let mut position = self.seed_position();
    let SearchOutcome {
      position: direction,
      mut expended_fuel,
    } = self.choose_direction(position);
    self.print_position(position, expended_fuel);
    let mut loop_count: i32 = 0;
    loop {
      if loop_count > LOOP_ESCAPE {
        panic!("Hill climb looped too many times");
      }
      loop_count += 1;
      let new_position = position + direction;
      let new_cost = self.compute_cost(new_position);
      if new_cost > expended_fuel {
        return SearchOutcome {
          position,
          expended_fuel,
        };
      }
      position = new_position;
      expended_fuel = new_cost;
      self.print_position(position, expended_fuel);
    }
  }
}

fn exhaustive_search(population: Vec<&Crab>, trace_search: bool) -> SearchOutcome {
  let start: i32 = population
    .iter()
    .map(|c| c.original_position)
    .min()
    .unwrap_or(0);
  let end: i32 = population
    .iter()
    .map(|c| c.original_position)
    .max()
    .unwrap_or(start);
  let mut position = start;
  let mut expended_fuel = compute_cost(&population, position);
  print_position(trace_search, position, expended_fuel);
  for i in start + 1..end {
    let new_cost = compute_cost(&population, i);
    print_position(trace_search, i, new_cost);
    if new_cost < expended_fuel {
      position = i;
      expended_fuel = new_cost;
    }
  }

  SearchOutcome {
    position,
    expended_fuel,
  }
}

fn execute_search(crabs: Vec<Crab>, config: Config) -> Result<SearchOutcome, String> {
  let trace_search = config.trace_search.unwrap_or(false);
  match config.search_method.as_str() {
    "hill_climb" => Ok(HillClimbAlg::new(crabs.iter().collect(), trace_search).execute()),
    "exhaustive_search" => Ok(exhaustive_search(crabs.iter().collect(), trace_search)),
    _ => Err(String::from("search_method not recognized")),
  }
}

fn parse_crabs(raw_crabs: String) -> Vec<Crab> {
  raw_crabs
    .split(",")
    .map(|v| v.parse::<i32>())
    .filter(|v| v.is_ok())
    .map(|v| v.unwrap())
    .map(|v| Crab::new(v))
    .collect()
}

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.data_file)?;
  let crabs = parse_crabs(raw_data);
  let outcome = execute_search(crabs, context.config)?;

  println!(
    "Best position is at {} with cost {}",
    outcome.position, outcome.expended_fuel,
  );

  Ok(())
}
