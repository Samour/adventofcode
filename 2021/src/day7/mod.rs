use serde::Deserialize;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

const LOOP_ESCAPE: i32 = 10_000;

#[derive(Deserialize)]
struct Config {
  trace_search: Option<bool>,
  data_file: String,
  search_method: String,
  cost_method: String,
}

type CostMethodology = fn(original_position: i32, target_position: i32) -> i32;

fn linear_cost(original_position: i32, target_position: i32) -> i32 {
  (original_position - target_position).abs()
}

fn incremental_cost(original_position: i32, target_position: i32) -> i32 {
  let lin_cost = linear_cost(original_position, target_position);
  lin_cost * (lin_cost + 1) / 2
}

struct Crab {
  original_position: i32,
  cost_methodology: CostMethodology,
}

impl Crab {
  fn new(original_position: i32, cost_methodology: CostMethodology) -> Crab {
    Crab {
      original_position,
      cost_methodology,
    }
  }

  fn compute_cost(&self, target_position: i32) -> i32 {
    (self.cost_methodology)(self.original_position, target_position)
  }
}

fn compute_cost(population: &Vec<&Crab>, position: i32) -> i32 {
  let mut cost = 0;
  for crab in population {
    cost += crab.compute_cost(position);
  }

  cost
}

fn print_position(trace_search: bool, position: i32, cost: i32, writer: &Writer) {
  if trace_search {
    writer.write(|| format!("Position = {}, Cost = {}", position, cost));
  }
}

struct SearchOutcome {
  position: i32,
  expended_fuel: i32,
}

struct HillClimbAlg<'a> {
  trace_climb: bool,
  writer: &'a Writer,
  population: Vec<&'a Crab>,
}

impl HillClimbAlg<'_> {
  fn new<'a>(population: Vec<&'a Crab>, trace_climb: bool, writer: &'a Writer) -> HillClimbAlg<'a> {
    HillClimbAlg {
      population,
      trace_climb,
      writer,
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
    print_position(self.trace_climb, position, cost, self.writer);
  }

  fn execute(&self) -> SearchOutcome {
    let mut position = self.seed_position();
    let SearchOutcome {
      position: direction,
      mut expended_fuel,
    } = self.choose_direction(position);
    self.print_position(position, expended_fuel);
    if direction == 0 {
      return SearchOutcome {
        position,
        expended_fuel,
      };
    }
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

fn exhaustive_search(population: Vec<&Crab>, trace_search: bool, writer: &Writer) -> SearchOutcome {
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
  print_position(trace_search, position, expended_fuel, writer);
  for i in start + 1..end {
    let new_cost = compute_cost(&population, i);
    print_position(trace_search, i, new_cost, writer);
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

fn execute_search(
  crabs: Vec<Crab>,
  config: Config,
  writer: &Writer,
) -> Result<SearchOutcome, String> {
  let trace_search = config.trace_search.unwrap_or(false);
  match config.search_method.as_str() {
    "hill_climb" => Ok(HillClimbAlg::new(crabs.iter().collect(), trace_search, writer).execute()),
    "exhaustive_search" => Ok(exhaustive_search(
      crabs.iter().collect(),
      trace_search,
      writer,
    )),
    _ => Err(String::from("search_method not recognized")),
  }
}

fn select_cost_method(cost_method: &str) -> Result<CostMethodology, String> {
  match cost_method {
    "linear" => Ok(linear_cost),
    "incremental" => Ok(incremental_cost),
    _ => Err(String::from("cost_method not recognised")),
  }
}

fn parse_crabs(raw_crabs: String, cost_method: &str) -> Result<Vec<Crab>, String> {
  let cost_methodology = select_cost_method(cost_method)?;
  Ok(
    raw_crabs
      .split(",")
      .map(|v| v.parse::<i32>())
      .filter(|v| v.is_ok())
      .map(|v| v.unwrap())
      .map(|v| Crab::new(v, cost_methodology))
      .collect(),
  )
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.data_file)?;
  let crabs = parse_crabs(raw_data, &context.config.cost_method)?;
  let outcome = execute_search(crabs, context.config, &writer)?;

  writer.write(|| {
    format!(
      "Best position is at {} with cost {}",
      outcome.position, outcome.expended_fuel,
    )
  });

  Ok(())
}
