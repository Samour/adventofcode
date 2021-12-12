use serde::Deserialize;
use std::collections::{HashMap, HashSet};

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

const NAME_START: &str = "start";
const NAME_END: &str = "end";

#[derive(Deserialize)]
struct Config {
  edges_file: String,
  permit_small_repeat: bool,
  debug: Option<bool>,
}

fn is_small_chamber(name: &String) -> bool {
  name.to_lowercase() == *name
}

struct GraphCrawl {
  edges: HashMap<String, Vec<String>>,
  permit_small_repeat: bool,
  // Cache
}

impl GraphCrawl {
  fn new(edges: HashMap<String, Vec<String>>, permit_small_repeat: bool) -> GraphCrawl {
    GraphCrawl {
      edges,
      permit_small_repeat,
    }
  }

  fn count_paths_to_end(&mut self, partial: Vec<String>) -> i32 {
    let last_node = match partial.get(partial.len() - 1) {
      Some(n) => n,
      None => return 0,
    };
    let mut has_small_repeat: bool = !self.permit_small_repeat;
    let mut past_small: HashSet<String> = HashSet::new();
    for node in &partial {
      if is_small_chamber(node) {
        if past_small.contains(node) {
          has_small_repeat = true;
        } else {
          past_small.insert(node.clone());
        }
      }
    }

    let mut result: i32 = 0;
    for next_step in self.edges.get(last_node).unwrap_or(&Vec::new()).clone() {
      if next_step == NAME_START || has_small_repeat && past_small.contains(&next_step) {
        continue;
      } else if next_step == NAME_END {
        result += 1;
        continue;
      }
      let mut path = partial.clone();
      path.push(next_step);
      result += self.count_paths_to_end(path);
    }

    result
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

  GraphCrawl::new(edges, config.permit_small_repeat)
}

fn count_paths(mut graph_crawl: GraphCrawl, writer: &Writer) -> i64 {
  let paths = graph_crawl.count_paths_to_end(vec![String::from(NAME_START)]);
  writer.write(|| format!("Paths found: {}", paths));
  paths as i64
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<i64, String> {
  let context: Context<Config> = factory.create()?;
  let raw_edges = context.load_data(&context.config.edges_file)?;
  let graph_crawl = parse_graph(raw_edges, &context.config);
  Ok(count_paths(graph_crawl, &writer))
}
