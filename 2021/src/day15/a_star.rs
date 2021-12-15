use std::collections::HashMap;

use crate::day15::config::Config;
use crate::writer::Writer;

const UNKNOWN_COST: i32 = 1_000_000;

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
    let mut queue: Vec<(i32, i32)> = vec![self.start_position];
    while !queue.is_empty() {
      let node = match queue.pop() {
        Some(n) => n,
        None => return Err(format!("Finished searching nodes without reaching target")),
      };
      let neighbours = self.get_adjacent_nodes(&node);
      let mut node_cost = self
        .path_risk
        .get(&node)
        .cloned()
        .ok_or_else(|| format!("node missing from risk map"))?;
      if node == self.target_position {
        return Ok(node_cost);
      }
      let mut did_push: bool = false;
      for n in neighbours {
        let n_new_cost = node_cost + self.specific_risk.get(&n).unwrap();
        if self.path_risk.get(&n).unwrap().clone() > n_new_cost {
          self.path_risk.insert(n, n_new_cost);
          queue.push(n);
          did_push = true;
          if self.debug {
            self
              .writer
              .write(|| format!("cost({}, {}) = {}", n.0, n.1, n_new_cost));
          }
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

pub fn find_path_cost(
  specific_risk: HashMap<(i32, i32), i32>,
  config: &Config,
  writer: &Writer,
) -> Result<i32, String> {
  let max_x = specific_risk.keys().map(|&(x, _)| x).max().unwrap_or(0);
  let max_y = specific_risk.keys().map(|&(_, y)| y).max().unwrap_or(0);
  PathFinding::create((0, 0), (max_x, max_y), specific_risk, config.debug, writer).build_path()
}
