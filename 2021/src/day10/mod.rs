use serde::Deserialize;
use std::collections::HashMap;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

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
  mode: String,
  symbol_scores: HashMap<char, i32>,
}

fn is_open(sym: char) -> bool {
  sym == SYM_OPEN_A || sym == SYM_OPEN_B || sym == SYM_OPEN_C || sym == SYM_OPEN_D
}

fn is_close(sym: char) -> bool {
  sym == SYM_CLOSE_A || sym == SYM_CLOSE_B || sym == SYM_CLOSE_C || sym == SYM_CLOSE_D
}

fn get_close_match(sym_open: char) -> Result<char, String> {
  if sym_open == SYM_OPEN_A {
    Ok(SYM_CLOSE_A)
  } else if sym_open == SYM_OPEN_B {
    Ok(SYM_CLOSE_B)
  } else if sym_open == SYM_OPEN_C {
    Ok(SYM_CLOSE_C)
  } else if sym_open == SYM_OPEN_D {
    Ok(SYM_CLOSE_D)
  } else {
    Err(String::from("Closing symbol not know"))
  }
}

// Outer result represents some error in parsing
// Inner result right = illegal char
// Inner result left = required closing chars
fn parse_line(line: &str) -> Result<Result<Vec<char>, char>, String> {
  let mut stack: Vec<char> = Vec::new();
  for c in line.chars() {
    if is_open(c) {
      stack.push(c);
    } else if is_close(c) {
      let matched = stack.pop();
      if matched.is_none() {
        return Ok(Err(c));
      }
      let matched = matched.unwrap();
      if get_close_match(matched)? != c {
        return Ok(Err(c));
      }
    } else {
      return Ok(Err(c));
    }
  }

  let mut closing: Vec<char> = Vec::new();
  loop {
    match stack.pop() {
      Some(c) => closing.push(get_close_match(c)?),
      None => break,
    }
  }

  Ok(Ok(closing))
}

fn score_completion(
  complete: Vec<char>,
  symbol_scores: &HashMap<char, i32>,
) -> Result<i64, String> {
  let mut result: i64 = 0;
  for c in complete {
    result *= 5;
    let c_score: i64 = symbol_scores
      .get(&c)
      .ok_or(String::from(
        "Closing character does not have an associated score",
      ))?
      .clone() as i64;
    result += c_score;
  }

  Ok(result)
}

fn compute_checker_score(raw_input: String, config: Config, writer: &Writer) -> Result<i64, String> {
  let mut result: i32 = 0;
  for line in raw_input.split("\n") {
    match parse_line(line)? {
      Err(c) => {
        result += config.symbol_scores.get(&c).ok_or(String::from(
          "Closing character does not have an associated score",
        ))?
      }
      _ => {}
    }
  }
  writer.write(|| format!("Checker score: {}", result));

  Ok(result as i64)
}

fn compute_complete_score(
  raw_input: String,
  config: Config,
  writer: &Writer,
) -> Result<i64, String> {
  let mut scores: Vec<i64> = Vec::new();
  for line in raw_input.split("\n") {
    match parse_line(line)? {
      Ok(c) => scores.push(score_completion(c, &config.symbol_scores)?),
      _ => {}
    }
  }

  scores.sort();
  let mid_score = scores
    .get(scores.len() / 2)
    .ok_or(String::from("Error when obtaining middle element of list"))?;
  writer.write(|| format!("Autocomplete score: {}", mid_score));

  Ok(*mid_score as i64)
}

fn select_mode(mode: &str) -> Result<fn(String, Config, &Writer) -> Result<i64, String>, String> {
  match mode {
    "checker_score" => Ok(compute_checker_score),
    "complete_score" => Ok(compute_complete_score),
    _ => Err(String::from("Mode not recognized")),
  }
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<String, String> {
  let context: Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.text_file)?;
  select_mode(&context.config.mode)?(raw_data, context.config, &writer).map(|r| format!("{}", r))
}
