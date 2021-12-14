use serde::Deserialize;
use std::collections::{HashMap, LinkedList};

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

#[derive(Deserialize)]
struct Config {
  rules_file: String,
  steps: i32,
  debug: Option<bool>,
}

struct InsertionRuleSet {
  rules: HashMap<(char, char), char>,
}

impl InsertionRuleSet {
  fn new(rules: HashMap<(char, char), char>) -> InsertionRuleSet {
    InsertionRuleSet { rules }
  }
}

fn increment_count<K: Eq + std::hash::Hash>(map: &mut HashMap<K, i64>, key: K, increment: i64) {
  let mut count = map.remove(&key).unwrap_or(0);
  count += increment;
  map.insert(key, count);
}

struct Polymer {
  first_char: char,
  last_char: char,
  elements: HashMap<(char, char), i64>,
}

impl Polymer {
  fn new(first_char: char, last_char: char, elements: HashMap<(char, char), i64>) -> Polymer {
    Polymer {
      first_char,
      last_char,
      elements,
    }
  }

  fn apply_rules(&self, rule_set: &InsertionRuleSet) -> Polymer {
    let mut elements: HashMap<(char, char), i64> = HashMap::new();
    for (&pair, &count) in &self.elements {
      match rule_set.rules.get(&pair) {
        Some(&insert) => {
          increment_count(&mut elements, (pair.0, insert), count);
          increment_count(&mut elements, (insert, pair.1), count);
        }
        None => increment_count(&mut elements, pair, count),
      }
    }

    Polymer::new(self.first_char, self.last_char, elements)
  }

  fn build_char_map(&self) -> HashMap<char, i64> {
    let mut result: HashMap<char, i64> = HashMap::new();
    let mut is_first: bool = true;
    for (pair, &count) in &self.elements {
      if is_first {}
      increment_count(&mut result, pair.0, count);
      increment_count(&mut result, pair.1, count);
      is_first = false;
    }
    increment_count(&mut result, self.first_char, 1);
    increment_count(&mut result, self.last_char, 1);

    result.into_iter().map(|(k, v)| (k, v / 2)).collect()
  }

  fn explain_elements(&self, writer: &Writer) {
    writer.write(|| format!("By pairs:"));
    for (pair, count) in &self.elements {
      writer.write(|| format!("({}, {}) -> {}", pair.0, pair.1, count));
    }
    writer.write(|| format!("By character:"));
    for (c, count) in self.build_char_map() {
      writer.write(|| format!("{} -> {}", c, count));
    }
  }
}

fn parse_polymer(raw_poly: &str) -> Option<Polymer> {
  let mut elements: HashMap<(char, char), i64> = HashMap::new();
  let mut first_c: Option<char> = None;
  let mut last_c: Option<char> = None;
  for c in raw_poly.trim().chars() {
    match last_c {
      Some(lc) => increment_count(&mut elements, (lc, c), 1),
      None => first_c = Some(c),
    }
    last_c = Some(c);
  }
  Some(Polymer::new(first_c?, last_c?, elements))
}

fn parse_rule(raw_rule: &str) -> Option<((char, char), char)> {
  let parts: Vec<&str> = raw_rule.split("->").collect();
  if parts.len() != 2 {
    return None;
  }
  let pattern: Vec<char> = parts[0].trim().chars().collect();
  let insert: Vec<char> = parts[1].trim().chars().collect();
  if pattern.len() != 2 || insert.len() != 1 {
    None
  } else {
    Some(((pattern[0], pattern[1]), insert[0]))
  }
}

fn parse(raw_rules: String) -> Result<(Polymer, InsertionRuleSet), String> {
  let mut polymer: Option<Polymer> = None;
  let mut rules: HashMap<(char, char), char> = HashMap::new();
  let mut line_no = 0;
  for line in raw_rules.split("\n") {
    match line_no {
      0 => polymer = parse_polymer(line),
      1 => {}
      _ => match parse_rule(line) {
        Some((pattern, insert)) => {
          rules.insert(pattern, insert);
        }
        None => {}
      },
    }
    line_no += 1;
  }

  Ok((
    polymer.ok_or_else(|| format!("Could not parse polymer template"))?,
    InsertionRuleSet::new(rules),
  ))
}

fn apply_steps(
  mut polymer: Polymer,
  rule_set: InsertionRuleSet,
  config: Config,
  writer: Writer,
) -> Result<i64, String> {
  let debug = config.debug.unwrap_or(false);
  for i in 0..config.steps {
    polymer = polymer.apply_rules(&rule_set);
    if debug {
      writer.write(|| format!("After step {}", i + 1));
      polymer.explain_elements(&writer);
      writer.write(|| format!(""));
    }
  }

  let mut counts: Vec<i64> = polymer.build_char_map().values().cloned().collect();
  let total_length: i64 = counts.iter().sum();
  // let total_length: i32 = polymer.elements.values().sum();
  counts.sort();
  let c_min = counts
    .get(0)
    .ok_or_else(|| format!("Could not find minimum element count"))?;
  let c_max = counts
    .get(counts.len() - 1)
    .ok_or_else(|| format!("Could not find maximum element count"))?;
  let result = c_max - c_min;
  writer.write(|| format!("Total length: {}", total_length));
  writer.write(|| format!("Element difference: {} - {} = {}", c_max, c_min, result));

  Ok(result)
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<String, String> {
  let context: Context<Config> = factory.create()?;
  let raw_rules = context.load_data(&context.config.rules_file)?;
  let (polymer, rule_set) = parse(raw_rules)?;
  apply_steps(polymer, rule_set, context.config, writer).map(|r| format!("{}", r))
}
