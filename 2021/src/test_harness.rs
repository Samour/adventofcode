use serde::Deserialize;
use std::time::Instant;

use crate::config::{Context, ContextFactory};
use crate::implementations::execute;

#[derive(Deserialize)]
struct TestCase {
  name: String,
  config: String,
  expected_result: i64,
}

#[derive(Deserialize)]
struct TestConfig {
  show_errors: Option<bool>,
  enable_printing: Option<bool>,
}

#[derive(Deserialize)]
struct TestSuite {
  tests: Vec<TestCase>,
  config: Option<TestConfig>,
}

struct TestOutcome<'a> {
  config: &'a TestCase,
  result: Result<(), String>,
  time: u128,
}

fn run_test<'a>(config_fname: String, config: &'a TestCase) -> TestOutcome<'a> {
  let start = Instant::now();
  let result = execute(config_fname);
  let time = start.elapsed().as_millis();

  TestOutcome {
    config,
    result,
    time,
  }
}

pub fn execute_tests(config_fname: String) -> Result<(), String> {
  let context: Context<TestSuite> = ContextFactory::new(config_fname).create()?;
  let mut results: Vec<TestOutcome> = Vec::new();
  for suite in &context.config.tests {
    results.push(match context.get_resource(&suite.config) {
      Ok(suite_fname) => run_test(suite_fname, suite),
      Err(e) => TestOutcome {
        config: suite,
        result: Err(e),
        time: 0,
      },
    })
  }

  // TODO some real formatting
  let show_errors = context
    .config
    .config
    .map(|c| c.show_errors)
    .flatten()
    .unwrap_or(false);
  println!("Test case                  Outcome          Time (ms)");
  for result in results {
    let outcome = match result.result {
      Ok(_) => "Completed",
      Err(_) => "Error",
    };
    println!(
      "{}        {}        {}",
      result.config.name, outcome, result.time
    );
    if show_errors && result.result.is_err() {
      println!("      {}", result.result.unwrap_err());
    }
  }

  Ok(())
}
