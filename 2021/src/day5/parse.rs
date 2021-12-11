use std::num::ParseIntError;

use crate::day5::map::{Coordinate, Line};

fn parse_coordinate(raw_coord: &str) -> Result<Coordinate, ParseIntError> {
  let parts: Vec<&str> = raw_coord.split(",").map(|p| p.trim()).collect();

  Ok(Coordinate::new(parts[0].parse()?, parts[1].parse()?))
}

pub fn parse_lines(raw_lines: String) -> Vec<Line> {
  let mut lines: Vec<Line> = Vec::new();
  for line in raw_lines.split("\n") {
    let coords: Vec<&str> = line.split("->").collect();
    if coords.len() < 2 {
      continue;
    }
    let start = parse_coordinate(coords[0]);
    let end = parse_coordinate(coords[1]);
    if start.is_ok() && end.is_ok() {
      lines.push(Line::new(start.unwrap(), end.unwrap()));
    }
  }

  lines
}
