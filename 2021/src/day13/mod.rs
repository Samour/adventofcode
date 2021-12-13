use regex::{Captures, Match, Regex};
use serde::Deserialize;
use std::collections::HashSet;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

#[derive(Deserialize)]
struct Config {
  data_file: String,
  folds_to_apply: i32,
  render_outcome: bool,
}

enum Fold {
  Horizontal(i32),
  Vertical(i32),
}

struct PaperPlane {
  dots: HashSet<(i32, i32)>,
}

fn transform_point(p: i32, t: i32) -> i32 {
  if p <= t {
    p
  } else {
    2 * t - p
  }
}

impl PaperPlane {
  fn new(dots: HashSet<(i32, i32)>) -> PaperPlane {
    PaperPlane { dots }
  }

  fn fold(&self, fold: Fold) -> PaperPlane {
    let mut dots: HashSet<(i32, i32)> = HashSet::new();
    for dot in &self.dots {
      dots.insert(match fold {
        Fold::Horizontal(y) => (dot.0, transform_point(dot.1, y)),
        Fold::Vertical(x) => (transform_point(dot.0, x), dot.1),
      });
    }

    PaperPlane::new(dots)
  }

  fn draw_plane(&self) -> Option<String> {
    let mut result = String::new();
    let x0 = self.dots.iter().map(|&(x, _)| x).min()?;
    let x1 = self.dots.iter().map(|&(x, _)| x).max()?;
    let y0 = self.dots.iter().map(|&(_, y)| y).min()?;
    let y1 = self.dots.iter().map(|&(_, y)| y).max()?;
    for y in y0..y1 + 1 {
      for x in x0..x1 + 1 {
        if self.dots.contains(&(x, y)) {
          result.push('#');
        } else {
          result.push('.');
        }
      }
      result.push('\n');
    }

    Some(result)
  }
}

fn to_coord(val: Option<Match>) -> Option<i32> {
  val?.as_str().parse::<i32>().ok()
}

fn to_point(capture: Captures) -> Option<(i32, i32)> {
  Some((to_coord(capture.get(1))?, to_coord(capture.get(2))?))
}

fn parse_input(raw_input: String) -> Result<(PaperPlane, Vec<Fold>), String> {
  let mut dots: HashSet<(i32, i32)> = HashSet::new();
  let mut folds: Vec<Fold> = Vec::new();
  let r_point =
    Regex::new(r"^([0-9]+),([0-9]+)$").map_err(|e| format!("Error parsing regex: {:?}", e))?;
  let r_fold = Regex::new(r"^fold along ([xy])=(\d+)$")
    .map_err(|e| format!("Error parsing regex: {:?}", e))?;
  for line in raw_input.split("\n") {
    match r_point.captures(line) {
      Some(m) => match to_point(m) {
        Some(point) => {
          dots.insert(point);
        }
        None => return Err(format!("Error while parsing point")),
      },
      None => match r_fold.captures(line) {
        Some(m) => match to_coord(m.get(2)) {
          Some(p) => match m.get(1).map(|c| c.as_str()) {
            Some("x") => folds.push(Fold::Vertical(p)),
            Some("y") => folds.push(Fold::Horizontal(p)),
            _ => return Err(format!("Error while parsing fold")),
          },
          None => return Err(format!("Error while parsing fold")),
        },
        _ => {}
      },
    }
  }

  Ok((PaperPlane::new(dots), folds))
}

fn perform_folds(
  mut plane: PaperPlane,
  folds: Vec<Fold>,
  config: Config,
  writer: Writer,
) -> Result<i32, String> {
  let mut i = 0;
  for fold in folds {
    if config.folds_to_apply >= 0 && i >= config.folds_to_apply {
      break;
    }
    i += 1;
    plane = plane.fold(fold);
  }

  writer.write(|| format!("Dots on final PaperPlane: {}", plane.dots.len()));
  if config.render_outcome {
    writer.write(|| plane.draw_plane().unwrap_or_else(String::new));
  }

  Ok(plane.dots.len() as i32)
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<String, String> {
  let context: Context<Config> = factory.create()?;
  let raw_input = context.load_data(&context.config.data_file)?;
  let (paper_plane, folds) = parse_input(raw_input)?;
  perform_folds(paper_plane, folds, context.config, writer).map(|r| format!("{}", r))
}
