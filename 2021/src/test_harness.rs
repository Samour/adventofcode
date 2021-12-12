use serde::Deserialize;
use std::time::Instant;

use crate::config::{Context, ContextFactory};
use crate::implementations::execute;
use crate::writer::Writer;

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
    println!("{:<80}{:<15}{:>10}", "Test case", "Outcome", "Time (ms)");
  }

  fn print_test_result(&self, result: &TestOutcome) {
    let outcome = match result.result {
      Ok(_) => "Completed",
      Err(_) => "Error",
    };
    println!(
      "{:<80}{:<15}{:>20}",
      result.config.name, outcome, result.time
    );
    if self.show_errors && result.result.is_err() {
      println!("       {}", result.result.as_ref().unwrap_err());
    }
  }

  fn print_aggregates(&self) {
    let mut c_passed: i32 = 0;
    let mut c_failed: i32 = 0;
    let mut c_error: i32 = 0;
    let mut total_time: u128 = 0;
    for result in &self.outcomes {
      match result.result {
        Ok(_) => c_passed += 1,
        Err(_) => c_error += 1,
      }
      total_time += result.time;
    }

    println!();
    println!("Tests passed: {}", c_passed);
    println!("Tests failed: {}", c_failed);
    println!("Test errors: {}", c_error);
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
