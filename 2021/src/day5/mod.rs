use serde::Deserialize;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

mod map;
mod parse;

use map::{count_lines_at_points, Line};
use parse::parse_lines;

#[derive(Deserialize)]
struct Config {
  data_file: String,
  apply_filter: bool,
}

fn filter_for_vertical(lines: Vec<Line>) -> Vec<Line> {
  lines
    .into_iter()
    .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
    .collect()
}

fn count_intersections(lines: Vec<Line>, writer: Writer) {
  let map_count = count_lines_at_points(lines.iter().collect());
  let mut intersection_count: i32 = 0;
  for &count in map_count.values() {
    if count > 1 {
      intersection_count += 1;
    }
  }

  writer.write(|| format!("Number of intersecting lines: {}", intersection_count));
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_lines = context.load_data(&context.config.data_file)?;
  let lines = parse_lines(raw_lines);

  count_intersections(
    if context.config.apply_filter {
      filter_for_vertical(lines)
    } else {
      lines
    },
    writer,
  );

  Ok(())
}
