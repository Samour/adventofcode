use serde::Deserialize;
use std::collections::{HashMap, HashSet};

use crate::config::{Context, ContextFactory};

const NAME_START: &str = "start";
const NAME_END: &str = "end";

#[derive(Deserialize)]
struct Config {
  edges_file: String,
  debug: Option<bool>,
}

fn is_small_chamber(name: String) -> bool {
  name.to_lowercase() == name
}

struct PartialPath {
  path: Vec<String>,
  next_steps: Vec<String>,
}

impl PartialPath {
  fn next_path(&mut self, edges: &HashMap<String, Vec<String>>) -> Option<PartialPath> {
    let next_step = self.next_steps.pop()?;
    let mut path = self.path.clone();
    path.push(next_step.clone());
    let excluded_nodes: HashSet<String> = path
      .iter()
      .cloned()
      .filter(|n| is_small_chamber(n.clone()))
      .collect();

    Some(PartialPath {
      path,
      next_steps: edges
        .get(&next_step)
        .unwrap_or(&Vec::new())
        .iter()
        .filter(|&s| !excluded_nodes.contains(s))
        .cloned()
        .collect(),
    })
  }

  fn is_end(&self) -> bool {
    self
      .path
      .get(self.path.len() - 1)
      .map(|n| n == NAME_END)
      .unwrap_or(false)
  }

  fn debug_out(&self) -> String {
    format!(
      "{{ [{}], [{}] }}",
      self.path.join(", "),
      self.next_steps.join(", "),
    )
  }
}

struct GraphCrawl {
  edges: HashMap<String, Vec<String>>,
  paths: Vec<PartialPath>,
  paths_found: i32,
  debug: bool,
}

impl GraphCrawl {
  fn new(edges: HashMap<String, Vec<String>>, debug: bool) -> GraphCrawl {
    let initial_path = PartialPath {
      path: vec![String::from(NAME_START)],
      next_steps: edges.get(NAME_START).cloned().unwrap_or(vec![]),
    };
    GraphCrawl {
      edges,
      paths: vec![initial_path],
      paths_found: 0,
      debug,
    }
  }

  fn search(&mut self) -> i32 {
    while self.step() {}
    self.paths_found
  }

  // false = no further iterations
  fn step(&mut self) -> bool {
    self.print_state();
    match self.paths.pop() {
      Some(mut path) => {
        if path.is_end() {
          self.paths_found += 1;
        } else {
          match path.next_path(&self.edges) {
            Some(next_path) => {
              self.paths.push(path);
              self.paths.push(next_path);
            }
            None => {}
          }
        }
        true
      }
      None => false,
    }
  }

  fn print_state(&self) {
    if !self.debug {
      return;
    }
    for path in &self.paths {
      println!("{}", path.debug_out());
    }
    println!();
  }
}

fn push_directed_edge(edges: &mut HashMap<String, Vec<String>>, node1: &str, node2: &str) {
  let mut nodes = edges.remove(node1).unwrap_or_else(Vec::new);
  nodes.push(String::from(node2));
  edges.insert(String::from(node1), nodes);
}

fn push_edge(edges: &mut HashMap<String, Vec<String>>, node1: &str, node2: &str) {
  push_directed_edge(edges, node1, node2);
  push_directed_edge(edges, node2, node1);
}

fn parse_graph(raw_edges: String, config: &Config) -> GraphCrawl {
  let mut edges: HashMap<String, Vec<String>> = HashMap::new();
  for raw_edge in raw_edges.split("\n") {
    let parts: Vec<&str> = raw_edge.split("-").collect();
    if parts.len() == 2 {
      push_edge(&mut edges, parts[0], parts[1]);
    }
  }

  GraphCrawl::new(edges, config.debug.unwrap_or(false))
}

fn count_paths(mut graph_crawl: GraphCrawl) {
  let paths = graph_crawl.search();
  println!("Paths found: {}", paths);
}

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_edges = context.load_data(&context.config.edges_file)?;
  let graph_crawl = parse_graph(raw_edges, &context.config);
  count_paths(graph_crawl);
  Ok(())
}
