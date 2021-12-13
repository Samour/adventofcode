use serde::Deserialize;
use std::collections::HashMap;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

#[derive(Deserialize)]
struct Config {
  map_file: String,
  mode: String,
  num_basins: Option<usize>,
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
  basin_cache: HashMap<(usize, usize), (usize, usize)>,
}

impl HeightMap {
  fn new(heights: Vec<Vec<i32>>) -> HeightMap {
    HeightMap {
      heights,
      basin_cache: HashMap::new(),
    }
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

  fn determine_basin(&mut self, x: usize, y: usize) -> Result<Option<(usize, usize)>, String> {
    let height = self.height_at_pos(x, y)?;
    if height == 9 {
      return Ok(None);
    }
    let cached = self.basin_cache.get(&(x, y));
    if cached.is_some() {
      return Ok(cached.cloned());
    }
    let mut lowest_neighbour: (usize, usize) = (x, y);
    let mut lowest_neighour_height: i32 = height;
    for (x1, y1) in select_neighbours(x, y) {
      let neighbour_height = self.height_at_pos(x1, y1).unwrap_or(i32::MAX);
      if neighbour_height < lowest_neighour_height {
        lowest_neighbour = (x1, y1);
        lowest_neighour_height = neighbour_height;
      }
    }
    if lowest_neighbour != (x, y) {
      lowest_neighbour = self
        .determine_basin(lowest_neighbour.0, lowest_neighbour.1)?
        .ok_or(String::from("Error during basin climb"))?;
    }

    self.basin_cache.insert((x, y), lowest_neighbour);
    Ok(Some(lowest_neighbour))
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

fn compute_basin_sizes(
  height_map: &mut HeightMap,
) -> Result<HashMap<(usize, usize), usize>, String> {
  let mut result: HashMap<(usize, usize), usize> = HashMap::new();
  for y in 0..height_map.heights.len() {
    for x in 0..height_map.heights[y].len() {
      match height_map.determine_basin(x, y)? {
        Some(basin) => {
          let mut count = result.remove(&basin).unwrap_or(0);
          count += 1;
          result.insert(basin, count);
        }
        _ => {}
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

fn sum_all_risk_scores(height_map: HeightMap, writer: &Writer) -> Result<i32, String> {
  let risk_sum: i32 = find_all_local_min(&height_map)?
    .into_iter()
    .map(|(x, y)| height_map.height_at_pos(x, y).unwrap() + 1)
    .sum();
  writer.write(|| format!("Sum of all risk scores: {}", risk_sum));

  Ok(risk_sum)
}

fn mult_top_basins(
  mut height_map: HeightMap,
  config: Config,
  writer: &Writer,
) -> Result<i32, String> {
  let num_basins = config
    .num_basins
    .ok_or(String::from("num_basins must be specified with this mode"))?;
  let mut basins_size: Vec<((usize, usize), usize)> =
    compute_basin_sizes(&mut height_map)?.into_iter().collect();
  basins_size.sort_by(|(_, s1), (_, s2)| s2.cmp(s1));
  let mut basins_total: i32 = 1;
  for &(_, size) in basins_size.iter().take(num_basins) {
    basins_total *= size as i32;
  }
  writer.write(|| format!("Multiple of basin sizes: {}", basins_total));

  Ok(basins_total)
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<String, String> {
  let context: Context<Config> = factory.create()?;
  let raw_map = context.load_data(&context.config.map_file)?;
  let height_map = parse_map(raw_map);
  match context.config.mode.as_str() {
    "locate_min" => sum_all_risk_scores(height_map, &writer),
    "locate_basins" => mult_top_basins(height_map, context.config, &writer),
    _ => Err(String::from("Unrecognized mode")),
  }
  .map(|r| format!("{}", r))
}
