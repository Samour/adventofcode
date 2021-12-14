use serde::Deserialize;
use std::collections::{HashMap, LinkedList};

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

#[derive(Deserialize)]
struct Config {
  rules_file: String,
  steps: i32,
  print_polymer: bool,
}

struct InsertionRuleSet {
  rules: HashMap<(char, char), char>,
}

impl InsertionRuleSet {
  fn new(rules: HashMap<(char, char), char>) -> InsertionRuleSet {
    InsertionRuleSet { rules }
  }
}

struct Polymer {
  elements: LinkedList<char>,
}

impl Polymer {
  fn new(elements: LinkedList<char>) -> Polymer {
    Polymer { elements }
  }

  fn apply_rules(&self, rule_set: &InsertionRuleSet) -> Polymer {
    let mut new_elements: LinkedList<char> = LinkedList::new();
    let mut last_char: Option<char> = None;
    for &c in &self.elements {
      match last_char {
        Some(lc) => match rule_set.rules.get(&(lc, c)) {
          Some(&insert) => new_elements.push_back(insert),
          None => {}
        },
        None => {}
      }
      new_elements.push_back(c);
      last_char = Some(c);
    }

    Polymer::new(new_elements)
  }

  fn render(&self) -> String {
    self.elements.iter().collect()
  }
}

fn parse_polymer(raw_poly: &str) -> Polymer {
  Polymer::new(raw_poly.trim().chars().collect())
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
      0 => polymer = Some(parse_polymer(line)),
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
) -> Result<i32, String> {
  for i in 0..config.steps {
    polymer = polymer.apply_rules(&rule_set);
    if config.print_polymer {
      writer.write(|| format!("Polymer: {}", polymer.render()));
    }
  }

  let mut total_length: i32 = 0;
  let mut char_count: HashMap<char, i32> = HashMap::new();
  for c in polymer.elements {
    total_length += 1;
    let mut count = char_count.remove(&c).unwrap_or(0);
    count += 1;
    char_count.insert(c, count);
  }
  let mut counts: Vec<i32> = char_count.values().cloned().collect();
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
