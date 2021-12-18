use serde::Deserialize;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

#[derive(Deserialize)]
struct TargetAreaConfig {
  min: i32,
  max: i32,
}

#[derive(Deserialize)]
struct Config {
  target_x: TargetAreaConfig,
  target_y: TargetAreaConfig,
  debug: Option<bool>,
}

#[derive(Clone)]
struct TargetArea {
  x_range: (i32, i32),
  y_range: (i32, i32),
}

struct ProbeAnalyzer<'a> {
  target_area: TargetArea,
  debug: bool,
  writer: &'a Writer,
}

impl ProbeAnalyzer<'_> {
  fn new<'a>(target_area: TargetArea, debug: bool, writer: &'a Writer) -> ProbeAnalyzer<'a> {
    ProbeAnalyzer {
      target_area,
      debug,
      writer,
    }
  }

  fn find_max_height(&self) -> i32 {
    let mut max_y: i32 = 0;
    let mut x0 = self.minimal_initial_x_v();
    // Loop over x0 values - step 2
    while x0 <= self.target_area.x_range.1 {
      let mut y0 = 1;
      'loop_steps: loop {
        // self.print_debug(|| format!("DEBUG 1"));
        let mut step = 0;
        let mut no_step_evaluations = true;
        while self.x_pos_at_step(x0, step) <= self.target_area.x_range.1
          && self.y_pos_at_step(y0, step - 1) >= self.target_area.y_range.0
        {
          no_step_evaluations = false;
          for offset in vec![-1, 0] {
            self.print_debug(|| format!("Considering initial_v {}, {} at step {}", x0, y0, step));
            let x_pos = self.x_pos_at_step(x0, step);
            let y_pos = self.y_pos_at_step(x0, step + offset);
            if x_pos >= self.target_area.x_range.0 && y_pos <= self.target_area.y_range.1 {
              let mut max_y_for_configuration = self.max_height_given_initial_y(y0);
              if max_y_for_configuration > max_y {
                self.print_debug(|| format!("Max height found: {}", max_y_for_configuration));
                max_y = max_y_for_configuration;
              }
            }
          }
          step += 1;
        }
        if no_step_evaluations {
          break;
        }
        y0 += 1;
      }
      x0 += 1;
    }

    max_y
  }

  fn minimal_initial_x_v(&self) -> i32 {
    (((8f64 * (self.target_area.x_range.0 as f64) + 1f64).sqrt() - 1f64) / 2f64).ceil() as i32
  }

  fn x_pos_at_step(&self, x_v: i32, step: i32) -> i32 {
    let l: i32 = vec![step, x_v - 1].into_iter().min().unwrap();
    l * x_v - (l * (l - 1)) / 2
  }

  fn max_height_given_initial_y(&self, y_v: i32) -> i32 {
    ((y_v + 1) * y_v) / 2
  }

  fn y_pos_at_step(&self, y_v: i32, step: i32) -> i32 {
    step * y_v - (step * (step - 1)) / 2
  }

  fn print_debug<F, S>(&self, output: F)
  where
    F: FnOnce() -> S,
    S: std::fmt::Display,
  {
    if self.debug {
      self.writer.write(output);
    }
  }
}

fn find_greatest_height(config: Config, writer: Writer) -> Result<String, String> {
  let target_area = TargetArea {
    x_range: (config.target_x.min, config.target_x.max),
    y_range: (config.target_y.min, config.target_y.max),
  };
  let result =
    ProbeAnalyzer::new(target_area, config.debug.unwrap_or(false), &writer).find_max_height();
  writer.write(|| format!("Greatest height found is {}", result));

  Ok(format!("{}", result))
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<String, String> {
  let context: Context<Config> = factory.create()?;
  find_greatest_height(context.config, writer)
}
