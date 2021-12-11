use serde::Deserialize;

use crate::config::{Context, ContextFactory};

#[derive(Deserialize)]
struct Config {
  map_file: String,
}

fn is_lower<T>(base: i32, comp: Result<i32, T>) -> bool {
  base < comp.unwrap_or(i32::MAX)
}

fn select_neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
  let mut result: Vec<(usize, usize)> = Vec::new();
  if x > 0 {
    result.push((x - 1, y));
  }
  result.push((x + 1, y));
  if y > 0 {
    result.push((x, y - 1));
  }
  result.push((x, y + 1));

  result
}

struct HeightMap {
  heights: Vec<Vec<i32>>,
}

impl HeightMap {
  fn new(heights: Vec<Vec<i32>>) -> HeightMap {
    HeightMap { heights }
  }

  fn height_at_pos(&self, x: usize, y: usize) -> Result<i32, String> {
    self
      .heights
      .get(y)
      .map(|v| v.get(x))
      .flatten()
      .cloned()
      .ok_or(String::from("Invalid coordinate specified"))
  }

  fn local_min_at_pos(&self, x: usize, y: usize) -> Result<bool, String> {
    let height = self.height_at_pos(x, y)?;
    Ok(
      select_neighbours(x, y)
        .into_iter()
        .all(|(x1, y1)| is_lower(height, self.height_at_pos(x1, y1))),
    )
  }
}

fn find_all_local_min(height_map: &HeightMap) -> Result<Vec<(usize, usize)>, String> {
  let mut result: Vec<(usize, usize)> = Vec::new();
  for y in 0..height_map.heights.len() {
    for x in 0..height_map.heights[y].len() {
      if height_map.local_min_at_pos(x, y)? {
        result.push((x, y));
      }
    }
  }

  Ok(result)
}

fn parse_map(raw_map: String) -> HeightMap {
  let heights: Vec<Vec<i32>> = raw_map
    .split("\n")
    .map(|l| l.trim())
    .filter(|l| l.len() > 0)
    .map(|l| {
      l.chars()
        .map(|c| String::from(c))
        .map(|c| c.parse::<i32>())
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .collect()
    })
    .collect();
  HeightMap::new(heights)
}

fn sum_all_risk_scores(height_map: HeightMap) -> Result<(), String> {
  let risk_sum: i32 = find_all_local_min(&height_map)?
    .into_iter()
    .map(|(x, y)| height_map.height_at_pos(x, y).unwrap() + 1)
    .sum();
  println!("Sum of all risk scores: {}", risk_sum);

  Ok(())
}

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_map = context.load_data(&context.config.map_file)?;
  let height_map = parse_map(raw_map);
  sum_all_risk_scores(height_map)?;

  Ok(())
}
