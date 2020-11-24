use crate::config::{Context, ContextFactory};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Deserialize)]
struct Config {
  path1: String,
  path2: String,
  dist_calc: String,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct PathCell {
  x: i32,
  y: i32,
  step_pos: u32,
}

impl PathCell {
  fn new(x: i32, y: i32, step_pos: u32) -> PathCell {
    PathCell { x, y, step_pos }
  }
}

struct Path {
  coordinates: HashMap<i32, HashMap<i32, PathCell>>,
}

impl Path {
  fn new() -> Path {
    Path {
      coordinates: HashMap::new(),
    }
  }

  fn add_coordinate(&mut self, cell: PathCell) -> () {
    if !self.coordinates.contains_key(&cell.x) {
      self.coordinates.insert(cell.x, HashMap::new());
    }
    self
      .coordinates
      .get_mut(&cell.x)
      .unwrap()
      .insert(cell.y, cell);
  }
}

fn construct_path(path: &str) -> Result<Path, String> {
  let mut x = 0;
  let mut y = 0;
  let mut step = 0;
  let mut result = Path::new();

  for mov in path.split(",") {
    let dir = mov.chars().take(1).collect::<String>();
    let dist = mov.chars().skip(1).collect::<String>().parse::<u32>();
    if let Err(_) = dist {
      return Err(format!("Failure parsing path"));
    }
    let dist = dist.unwrap();

    for _ in 0..dist {
      match &dir[..] {
        "R" => x += 1,
        "L" => x -= 1,
        "U" => y += 1,
        "D" => y -= 1,
        _ => return Err(format!("Unknown direction specifier: {}", dir)),
      }
      step += 1;
      result.add_coordinate(PathCell::new(x, y, step));
    }
  }

  Ok(result)
}

fn extract_intersections(path1: &Path, path2: &Path) -> HashSet<(PathCell, PathCell)> {
  let mut result = HashSet::<(PathCell, PathCell)>::new();
  for (x, ys) in &path1.coordinates {
    if !path2.coordinates.contains_key(x) {
      continue;
    }
    for (y, cell) in ys {
      let cell2 = path2.coordinates.get(x).map(|y2s| y2s.get(y)).flatten();
      if let Some(c2) = cell2 {
        result.insert((cell.clone(), c2.clone()));
      }
    }
  }

  result
}

fn mh_dist((cell, _): &(PathCell, PathCell)) -> u32 {
  (cell.x.abs() + cell.y.abs()) as u32
}

fn step_dist((cell1, cell2): &(PathCell, PathCell)) -> u32 {
  cell1.step_pos + cell2.step_pos
}

fn select_distance(dist_calc: &str) -> Result<fn(cells: &(PathCell, PathCell)) -> u32, String> {
  match dist_calc {
    "mh" => Ok(mh_dist),
    "step" => Ok(step_dist),
    _ => Err(format!("Unrecognized distance calculator: '{}'", dist_calc)),
  }
}

pub fn main(context_factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = context_factory.create()?;
  let path1 = construct_path(&context.config.path1)?;
  let path2 = construct_path(&context.config.path2)?;

  let intersections = extract_intersections(&path1, &path2);
  for (c1, c2) in &intersections {
    println!(
      "INFO: Intersection at ({}, {}, step1={}, step2={})",
      c1.x, c1.y, c1.step_pos, c2.step_pos,
    );
  }

  let dist = intersections
    .iter()
    .map(select_distance(&context.config.dist_calc)?)
    .min();

  if let Some(d) = dist {
    println!("Closest intersection has distance {}", d);
  } else {
    println!("No intersection found");
  }

  Ok(())
}
