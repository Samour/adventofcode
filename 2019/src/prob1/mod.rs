use crate::config::{Context, ContextFactory};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
  data_file: String,
  consider_fuel_weight: bool,
}

fn module_fuel(mass: i32) -> i32 {
  mass / 3 - 2
}

fn module_fuel_with_mass(mass: i32) -> i32 {
  let result = module_fuel(mass);
  if result < 0 {
    return 0;
  } else {
    result + module_fuel_with_mass(result)
  }
}

fn select_fuel_impl(consider_fuel_weight: bool) -> fn(mass: i32) -> i32 {
  if consider_fuel_weight {
    module_fuel_with_mass
  } else {
    module_fuel
  }
}

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;

  let data = context.load_data(&context.config.data_file)?;

  let fuel_impl = select_fuel_impl(context.config.consider_fuel_weight);
  let mut total_fuel = 0;
  for value in data.split("\n") {
    let mass = value.parse::<i32>();
    if let Ok(m) = mass {
      let fuel = fuel_impl(m);
      println!("INFO: mass={}; fuel={}", m, fuel);
      total_fuel += fuel;
    }
  }

  println!("Total fuel: {}", total_fuel);

  Ok(())
}
