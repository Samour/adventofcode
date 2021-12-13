use serde::Deserialize;
use std::time::Instant;

use crate::config::{Context, ContextFactory};
use crate::implementations::execute;
use crate::writer::Writer;

fn colour_if(condition: bool, colour: &str) -> &str {
  if condition {
    colour
  } else {
    ""
  }
}

#[derive(Deserialize)]
struct TestCase {
  name: String,
  config: String,
  expected_result: String,
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
  result: Result<String, String>,
  time: u128,
}

struct TestResultAggregator<'a> {
  print_immediate: bool,
  show_errors: bool,
  outcomes: Vec<TestOutcome<'a>>,
}

impl<'a> TestResultAggregator<'a> {
  fn new(print_immediate: bool, show_errors: bool) -> TestResultAggregator<'a> {
    TestResultAggregator {
      print_immediate,
      show_errors,
      outcomes: Vec::new(),
    }
  }

  fn print_results_header(&self) {
    println!("{:<80}{:<20}{:>15}", "Test case", "Output", "Time (ms)");
  }

  fn print_test_result(&self, result: &TestOutcome) {
    let outcome = match &result.result {
      Ok(r) => {
        if *r == result.config.expected_result {
          format!("\x1b[32m{}\x1b[0m", r)
        } else {
          format!("\x1b[31m{}\x1b[0m", r)
        }
      }
      Err(_) => format!("\x1b[31mError\x1b[0m"),
    };
    // Need extra padding for non-printed characters
    println!(
      "{:<80}{:<29}{:>15}",
      result.config.name, outcome, result.time
    );
    if self.show_errors && result.result.is_err() {
      println!(
        "       \x1b[31m{}\x1b[0m",
        result.result.as_ref().unwrap_err()
      );
    }
  }

  fn print_aggregates(&self) {
    let mut c_passed: i32 = 0;
    let mut c_failed: i32 = 0;
    let mut c_error: i32 = 0;
    let mut total_time: u128 = 0;
    for result in &self.outcomes {
      match &result.result {
        Ok(r) => {
          if *r == result.config.expected_result {
            c_passed += 1;
          } else {
            c_failed += 1;
          }
        }
        Err(_) => c_error += 1,
      }
      total_time += result.time;
    }

    println!();
    println!(
      "{}Tests passed: {}\x1b[0m",
      colour_if(c_passed > 0, "\x1b[32m"),
      c_passed
    );
    println!(
      "{}Tests failed: {}\x1b[0m",
      colour_if(c_failed > 0, "\x1b[31m"),
      c_failed
    );
    println!(
      "{}Test errors: {}\x1b[0m",
      colour_if(c_error > 0, "\x1b[31m"),
      c_error
    );
    println!("Total execution time: {} ms", total_time);
  }

  fn initialise(&self) {
    if self.print_immediate {
      self.print_results_header();
    }
  }

  fn emit_result(&mut self, outcome: TestOutcome<'a>) {
    if self.print_immediate {
      self.print_test_result(&outcome);
    }
    self.outcomes.push(outcome);
  }

  fn finalise(&self) {
    if !self.print_immediate {
      self.print_results_header();
      for result in &self.outcomes {
        self.print_test_result(result);
      }
    }
    self.print_aggregates();
  }
}

fn run_test<'a>(
  config_fname: String,
  config: &'a TestCase,
  enable_printing: bool,
) -> TestOutcome<'a> {
  let writer = if enable_printing {
    println!("Starting test: {}", config.name);
    Writer::StdoutWriter
  } else {
    Writer::NoopWriter
  };
  let start = Instant::now();
  let result = execute(config_fname, writer);
  let time = start.elapsed().as_millis();
  if enable_printing {
    println!();
  }

  TestOutcome {
    config,
    result,
    time,
  }
}

pub fn execute_tests(config_fname: String) -> Result<(), String> {
  let context: Context<TestSuite> = ContextFactory::new(config_fname).create()?;
  let show_errors = context
    .config
    .config
    .as_ref()
    .map(|c| c.show_errors)
    .flatten()
    .unwrap_or(false);
  let enable_printing = context
    .config
    .config
    .as_ref()
    .map(|c| c.enable_printing)
    .flatten()
    .unwrap_or(false);
  let mut results: TestResultAggregator = TestResultAggregator::new(!enable_printing, show_errors);
  results.initialise();
  for suite in &context.config.tests {
    results.emit_result(match context.get_resource(&suite.config) {
      Ok(suite_fname) => run_test(suite_fname, suite, enable_printing),
      Err(e) => TestOutcome {
        config: suite,
        result: Err(e),
        time: 0,
      },
    })
  }
  results.finalise();

  Ok(())
}
