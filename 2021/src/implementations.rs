use serde::Deserialize;

use crate::config::ContextFactory;
use crate::writer::Writer;

#[derive(Deserialize)]
struct Config {
  implementation: String,
}

fn select_impl(name: &str) -> Option<fn(factory: ContextFactory, writer: Writer) -> Result<String, String>> {
  match name {
    "day1" => Some(crate::day1::main),
    "day2" => Some(crate::day2::main),
    "day3" => Some(crate::day3::main),
    "day4" => Some(crate::day4::main),
    "day5" => Some(crate::day5::main),
    "day6" => Some(crate::day6::main),
    "day7" => Some(crate::day7::main),
    "day8" => Some(crate::day8::main),
    "day9" => Some(crate::day9::main),
    "day10" => Some(crate::day10::main),
    "day11" => Some(crate::day11::main),
    "day12" => Some(crate::day12::main),
    "day13" => Some(crate::day13::main),
    "day14" => Some(crate::day14::main),
    "day15" => Some(crate::day15::main),
    "day16" => Some(crate::day16::main),
    "day17" => Some(crate::day17::main),
    "day18" => Some(crate::day18::main),
    "day19" => Some(crate::day19::main),
    _ => None,
  }
}

pub fn execute(config_fname: String, writer: Writer) -> Result<String, String> {
  let context_factory = ContextFactory::new(config_fname);
  let config: Config = context_factory.create()?.config;

  let problem_impl = select_impl(&config.implementation);

  if problem_impl.is_none() {
    return Err(format!(
      "No implementation found for {}",
      &config.implementation
    ));
  }

  problem_impl.unwrap()(context_factory, writer)
}
