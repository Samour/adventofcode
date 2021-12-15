use std::collections::HashMap;

use crate::day15::config::Config;
use crate::writer::Writer;

const UNKNOWN_COST: i32 = 1_000_000;

struct MinHeap {
  elements: Vec<(i32, (i32, i32))>,
}

impl MinHeap {
  fn new() -> MinHeap {
    MinHeap {
      elements: Vec::new(),
    }
  }

  fn is_empty(&self) -> bool {
    self.elements.is_empty()
  }

  fn swap(&mut self, i: usize, j: usize) {
    let swap = self.elements[i];
    self.elements[i] = self.elements[j];
    self.elements[j] = swap;
  }

  fn push_element(&mut self, item: (i32, i32), score: i32) {
    self.elements.push((score, item));
    self.float_node(self.elements.len());
  }

  fn float_node(&mut self, index: usize) {
    if index == 1 || self.elements[index - 1].0 > self.elements[index / 2 - 1].0 {
      return;
    }
    self.swap(index / 2 - 1, index - 1);
    self.float_node(index / 2);
  }

  fn pop_element(&mut self) -> Option<(i32, i32)> {
    if self.elements.len() <= 1 {
      return self.elements.pop().map(|(_, n)| n);
    }
    let result = self.elements[0].1;
    self.elements[0] = self.elements.pop()?;
    self.sink_node(1);
    Some(result)
  }

  fn sink_node(&mut self, index: usize) {
    if index * 2 > self.elements.len() {
      return;
    }
    if index * 2 == self.elements.len() {
      if self.elements[index - 1].0 >= self.elements[index * 2 - 1].0 {
        self.swap(index - 1, index * 2 - 1);
        self.sink_node(index * 2);
      }
    } else if self.elements[index - 1].0 >= self.elements[index * 2 - 1].0
      || self.elements[index - 1].0 >= self.elements[index * 2].0
    {
      if self.elements[index * 2 - 1] < self.elements[index * 2] {
        self.swap(index - 1, index * 2 - 1);
        self.sink_node(index * 2);
      } else {
        self.swap(index - 1, index * 2);
        self.sink_node(index * 2 + 1);
      }
    }
  }
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
    let mut heap: MinHeap = MinHeap::new();
    heap.push_element((0, 0), 0);
    while !heap.is_empty() {
      let node = match heap.pop_element() {
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
          heap.push_element(n, self.score_queue_item(&n));
          did_push = true;
          if self.debug {
            self
              .writer
              .write(|| format!("cost({}, {}) = {}", n.0, n.1, n_new_cost));
          }
        }
      }
    }

    Err(format!("path to target position not found"))
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
