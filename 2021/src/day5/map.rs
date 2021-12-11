use std::collections::HashMap;

const MAX_LOOP: i32 = 10_000;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
  pub x: i32,
  pub y: i32,
}

impl Coordinate {
  pub fn new(x: i32, y: i32) -> Coordinate {
    Coordinate { x, y }
  }
}

pub struct Line {
  pub start: Coordinate,
  pub end: Coordinate,
}

impl Line {
  pub fn new(start: Coordinate, end: Coordinate) -> Line {
    Line { start, end }
  }

  fn intersect_coordinates(&self) -> Vec<Coordinate> {
    let mut current_coordinate = self.start;
    let mut coordinates: Vec<Coordinate> = Vec::new();
    coordinates.push(current_coordinate);
    let mut loop_escape: i32 = 0;
    loop {
      // Panic instead of hang application on bug
      if loop_escape >= MAX_LOOP {
        panic!("Looped too many times in Line::iter_coordinates");
      }
      loop_escape += 1;

      let d_x = self.end.x - current_coordinate.x;
      let d_y = self.end.y - current_coordinate.y;
      if d_x.abs() > d_y.abs() {
        current_coordinate.x += d_x.signum();
      } else if d_x.abs() < d_y.abs() {
        current_coordinate.y += d_y.signum();
      } else if d_x != 0 {
        current_coordinate.x += d_x.signum();
        current_coordinate.y += d_y.signum();
      } else {
        break;
      }
      coordinates.push(current_coordinate);
    }

    coordinates
  }
}

pub fn count_lines_at_points(lines: Vec<&Line>) -> HashMap<Coordinate, i32> {
  let mut result = HashMap::new();
  for line in lines {
    for coordinate in line.intersect_coordinates() {
      let mut count = result.remove(&coordinate).unwrap_or(0);
      count += 1;
      result.insert(coordinate, count);
    }
  }

  result
}
