use serde::Deserialize;
use std::collections::HashMap;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

const UNKNOWN_COST: i32 = 1_000_000;

#[derive(Deserialize)]
struct Config {
  risk_map_file: String,
  mult_factor: i32,
  debug: bool,
}

struct PathFinding<'a> {
  start_position: (i32, i32),
  target_position: (i32, i32),
  specific_risk: HashMap<(i32, i32), i32>,
  path_risk: HashMap<(i32, i32), i32>,
  debug: bool,
  writer: &'a Writer,
}

impl PathFinding<'_> {
  fn create<'a>(
    start_position: (i32, i32),
    target_position: (i32, i32),
    specific_risk: HashMap<(i32, i32), i32>,
    debug: bool,
    writer: &'a Writer,
  ) -> PathFinding<'a> {
    let mut path_risk: HashMap<(i32, i32), i32> =
      specific_risk.keys().map(|&p| (p, UNKNOWN_COST)).collect();
    path_risk.insert(start_position, 0);

    PathFinding {
      start_position,
      target_position,
      specific_risk,
      path_risk,
      debug,
      writer,
    }
  }

  fn get_adjacent_nodes(&self, node: &(i32, i32)) -> Vec<(i32, i32)> {
    vec![
      (node.0 - 1, node.1),
      (node.0 + 1, node.1),
      (node.0, node.1 - 1),
      (node.0, node.1 + 1),
    ]
    .into_iter()
    .filter(|p| self.specific_risk.contains_key(p))
    .collect()
  }

  fn build_path(&mut self) -> Result<i32, String> {
    let mut queue: Vec<(i32, i32)> = self.get_adjacent_nodes(&self.start_position);
    queue = self.sort_queue(queue);
    while !queue.is_empty() {
      let node = match queue.pop() {
        Some(n) => n,
        None => return Err(format!("Finished searching nodes without reaching target")),
      };
      let neighbours = self.get_adjacent_nodes(&node);
      let new_cost = neighbours
        .iter()
        .map(|n| self.path_risk.get(n).unwrap())
        .min()
        .ok_or_else(|| format!("Node referenced without any cost"))?
        + self
          .specific_risk
          .get(&node)
          .ok_or_else(|| format!("Node referenced without any cost"))?;
      if self.debug {
        self
          .writer
          .write(|| format!("cost({}, {}) = {}", node.0, node.1, new_cost));
      }
      if node == self.target_position {
        return Ok(new_cost);
      }
      self.path_risk.insert(node, new_cost);
      let mut did_push: bool = false;
      for n in neighbours {
        let n_new_cost = new_cost + self.specific_risk.get(&n).unwrap();
        if self.path_risk.get(&n).unwrap().clone() > n_new_cost {
          self.path_risk.insert(n, n_new_cost);
          queue.push(n);
          did_push = true;
        }
      }
      if did_push {
        queue = self.sort_queue(queue);
      }
    }

    Err(format!("path to target position not found"))
  }

  fn sort_queue(&self, mut queue: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    queue.sort_by(|a, b| self.score_queue_item(b).cmp(&self.score_queue_item(a)));
    let mut result: Vec<(i32, i32)> = Vec::new();
    for n in queue {
      if result.is_empty() || result[result.len() - 1] != n {
        result.push(n);
      }
    }

    result
  }

  fn score_queue_item(&self, node: &(i32, i32)) -> i32 {
    let distance_from_goal =
      (node.0 - self.target_position.0).abs() + (node.1 - self.target_position.1).abs();
    self.path_risk.get(node).unwrap_or(&UNKNOWN_COST) + distance_from_goal
  }
}

fn parse_risk_map<'a>(
  raw_map: String,
  config: &Config,
  writer: &'a Writer,
) -> Result<PathFinding<'a>, String> {
  let mut specific_risk: HashMap<(i32, i32), i32> = HashMap::new();
  let mut x: i32 = 0;
  let mut y: i32 = 0;
  for line in raw_map.split("\n") {
    if line.len() == 0 {
      break;
    }
    x = 0;
    for c in line.chars() {
      specific_risk.insert(
        (x, y),
        format!("{}", c)
          .parse()
          .map_err(|e| format!("Failure while parsing risk map"))?,
      );
      x += 1;
    }
    y += 1;
  }

  for i in 0..config.mult_factor {
    for j in 0..config.mult_factor {
      if i == 0 && j == 0 {
        continue;
      }
      for x1 in 0..x {
        for y1 in 0..y {
          specific_risk.insert(
            (i * x + x1, j * y + y1),
            (specific_risk.get(&(x1, y1)).unwrap() + i + j - 1) % 9 + 1,
          );
        }
      }
    }
  }

  Ok(PathFinding::create(
    (0, 0),
    (x * config.mult_factor - 1, y * config.mult_factor - 1),
    specific_risk,
    config.debug,
    writer,
  ))
}

fn find_path(mut path_finding: PathFinding, writer: &Writer) -> Result<i32, String> {
  let result = path_finding.build_path()?;
  writer.write(|| format!("Cost of least risky path: {}", result));

  Ok(result)
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<String, String> {
  let context: Context<Config> = factory.create()?;
  let raw_map = context.load_data(&context.config.risk_map_file)?;
  let path_finding = parse_risk_map(raw_map, &context.config, &writer)?;
  find_path(path_finding, &writer).map(|r| format!("{}", r))
}
