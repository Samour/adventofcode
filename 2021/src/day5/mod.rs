use serde::Deserialize;

use crate::config::{Context, ContextFactory};

mod map;
mod parse;

use map::{count_lines_at_points, Line};
use parse::parse_lines;

#[derive(Deserialize)]
struct Config {
  data_file: String,
}

fn count_horiz_vert_intersections(lines: Vec<Line>) {
  let filtered_lines: Vec<&Line> = lines
    .iter()
    .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
    .collect();

  let map_count = count_lines_at_points(filtered_lines);
  let mut intersection_count: i32 = 0;
  for &count in map_count.values() {
    if count > 1 {
      intersection_count += 1;
    }
  }

  println!("Number of intersecting lines: {}", intersection_count);
}

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_lines = context.load_data(&context.config.data_file)?;
  let lines = parse_lines(raw_lines);

  count_horiz_vert_intersections(lines);

  Ok(())
}
