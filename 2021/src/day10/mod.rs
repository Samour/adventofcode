use serde::Deserialize;
use std::collections::HashMap;

use crate::config::{Context, ContextFactory};

const SYM_OPEN_A: char = '(';
const SYM_OPEN_B: char = '[';
const SYM_OPEN_C: char = '{';
const SYM_OPEN_D: char = '<';
const SYM_CLOSE_A: char = ')';
const SYM_CLOSE_B: char = ']';
const SYM_CLOSE_C: char = '}';
const SYM_CLOSE_D: char = '>';

#[derive(Deserialize)]
struct Config {
  text_file: String,
  symbol_scores: HashMap<char, i32>,
}

fn is_open(sym: char) -> bool {
  sym == SYM_OPEN_A || sym == SYM_OPEN_B || sym == SYM_OPEN_C || sym == SYM_OPEN_D
}

fn is_close(sym: char) -> bool {
  sym == SYM_CLOSE_A || sym == SYM_CLOSE_B || sym == SYM_CLOSE_C || sym == SYM_CLOSE_D
}

fn pair_match(sym_open: char, sym_close: char) -> bool {
  if sym_open == SYM_OPEN_A {
    sym_close == SYM_CLOSE_A
  } else if sym_open == SYM_OPEN_B {
    sym_close == SYM_CLOSE_B
  } else if sym_open == SYM_OPEN_C {
    sym_close == SYM_CLOSE_C
  } else if sym_open == SYM_OPEN_D {
    sym_close == SYM_CLOSE_D
  } else {
    false
  }
}

// Empty option = valid line, Some(n) = n is illegal character
fn parse_line(line: &str) -> Option<char> {
  let mut stack: Vec<char> = Vec::new();
  for c in line.chars() {
    if is_open(c) {
      stack.push(c);
    } else if is_close(c) {
      let matched = stack.pop();
      if matched.is_none() {
        return Some(c);
      }
      let matched = matched.unwrap();
      if !pair_match(matched, c) {
        return Some(c);
      }
    } else {
      return Some(c);
    }
  }

  None
}

fn compute_checker_score(raw_input: String, config: Config) -> Result<(), String> {
  let mut result: i32 = 0;
  for line in raw_input.split("\n") {
    match parse_line(line) {
      Some(c) => {
        result += config.symbol_scores.get(&c).ok_or(String::from(
          "Closing character does not have an associated score",
        ))?
      }
      _ => {}
    }
  }
  println!("Checker score: {}", result);

  Ok(())
}

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.text_file)?;
  compute_checker_score(raw_data, context.config)
}
