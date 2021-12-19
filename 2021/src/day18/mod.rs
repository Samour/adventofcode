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
  print_each_addition: Option<bool>,
  trace_reductions: Option<bool>,
}

fn parse_numbers(raw_numbers: &str) -> Result<VecDeque<Rc<RefCell<SnailFishNumber>>>, String> {
  let mut result: VecDeque<Rc<RefCell<SnailFishNumber>>> = VecDeque::new();
  for line in raw_numbers.split("\n") {
    if line.trim().len() > 0 {
      result.push_back(parse(String::from(line.trim()))?);
    }
  }

  Ok(result)
}

fn add_numbers(
  mut numbers: VecDeque<Rc<RefCell<SnailFishNumber>>>,
  writer: &Writer,
  print_each_addition: bool,
  trace_reductions: bool,
) -> Result<String, String> {
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

pub fn main(factory: ContextFactory, writer: Writer) -> Result<String, String> {
  let context: Context<Config> = factory.create()?;
  let numbers_raw = context.load_data(&context.config.numbers_file)?;
  let numbers = parse_numbers(&numbers_raw)?;
  add_numbers(
    numbers,
    &writer,
    context.config.print_each_addition.unwrap_or(false),
    context.config.trace_reductions.unwrap_or(false),
  )
}
