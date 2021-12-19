use serde::Deserialize;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

mod parser;
mod reducer;
mod sfn;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

use parser::parse;
use reducer::reduce;
use sfn::{plus, SnailFishNumber};

#[derive(Deserialize)]
struct Config {
  numbers_file: String,
  mode: String,
  print_each_addition: Option<bool>,
  trace_reductions: Option<bool>,
}

fn parse_numbers(
  raw_numbers: Vec<String>,
) -> Result<VecDeque<Rc<RefCell<SnailFishNumber>>>, String> {
  let mut result: VecDeque<Rc<RefCell<SnailFishNumber>>> = VecDeque::new();
  for line in raw_numbers {
    result.push_back(parse(String::from(line))?);
  }

  Ok(result)
}

fn add_numbers(
  numbers: Vec<String>,
  writer: &Writer,
  print_each_addition: bool,
  trace_reductions: bool,
) -> Result<String, String> {
  let mut numbers = parse_numbers(numbers)?;
  let mut base_number = numbers
    .pop_front()
    .ok_or_else(|| format!("No numbers in file"))?;
  while let Some(next_number) = numbers.pop_front() {
    if print_each_addition {
      writer.write(|| format!(" = {}", base_number.borrow().render()));
    }
    base_number = plus(base_number, next_number);
    if print_each_addition {
      writer.write(|| format!(" = {}", base_number.borrow().render()));
    }
    reduce(Rc::clone(&base_number), trace_reductions, writer)?;
  }
  writer.write(|| {
    format!(
      "Final value after additions & reductions: {}",
      base_number.borrow().render()
    )
  });
  let magnitude = base_number.borrow().content.magnitude();
  writer.write(|| format!("Magnitude: {}", magnitude));

  Ok(format!("{}", magnitude))
}

fn compute_magnitude(
  one: Rc<RefCell<SnailFishNumber>>,
  two: Rc<RefCell<SnailFishNumber>>,
  writer: &Writer,
) -> Result<i32, String> {
  let result = plus(one, two);
  reduce(Rc::clone(&result), false, writer)?;
  let result = result.borrow().content.magnitude();
  Ok(result)
}

fn find_max_magnitude(numbers: Vec<String>, writer: &Writer) -> Result<String, String> {
  let mut max_mag: i32 = 0;
  for i in 0..numbers.len() {
    for j in 0..numbers.len() {
      if i == j {
        continue;
      }
      let number1 = parse(numbers[i].clone())?;
      let number2 = parse(numbers[j].clone())?;
      let mag = compute_magnitude(number1, number2, writer)?;
      if mag > max_mag {
        max_mag = mag;
      }
    }
  }
  writer.write(|| format!("Maximum magnitude found: {}", max_mag));

  Ok(format!("{}", max_mag))
}

fn compute_numbers(
  numbers: Vec<String>,
  config: &Config,
  writer: &Writer,
) -> Result<String, String> {
  match config.mode.as_str() {
    "add_all" => add_numbers(
      numbers,
      writer,
      config.print_each_addition.unwrap_or(false),
      config.trace_reductions.unwrap_or(false),
    ),
    "max_pair" => find_max_magnitude(numbers, writer),
    _ => Err(format!("Unrecognized mode")),
  }
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<String, String> {
  let context: Context<Config> = factory.create()?;
  let numbers_raw: Vec<String> = context
    .load_data(&context.config.numbers_file)?
    .split("\n")
    .map(String::from)
    .collect();
  compute_numbers(numbers_raw, &context.config, &writer)
}
