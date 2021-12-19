use ndarray::{arr1, Array1};

use crate::day19::scanner::Scanner;

fn parse_point(raw_point: &str) -> Result<Array1<i32>, String> {
  let parts: Vec<i32> = raw_point
    .split(",")
    .map(|p| p.parse::<i32>())
    .filter(|p| p.is_ok())
    .map(|p| p.unwrap())
    .collect();

  if parts.len() != 3 {
    Err(format!("Error parsing point: wrong number of axes"))
  } else {
    Ok(arr1(&[parts[0], parts[1], parts[2]]))
  }
}

pub fn parse(raw_scanner: &str) -> Result<Vec<Scanner>, String> {
  let mut result: Vec<Scanner> = Vec::new();
  let mut name: Option<String> = None;
  let mut points: Vec<Array1<i32>> = Vec::new();
  for line in raw_scanner.split("\n") {
    if name.is_none() {
      if line.len() < 6 {
        return Err(format!("Malformed scanner header"));
      }
      name = Some(String::from(
        line
          .chars()
          .skip(3)
          .take(line.len() - 6)
          .collect::<String>()
          .trim(),
      ));
    } else if line.trim().len() == 0 {
      result.push(Scanner::create(
        name.ok_or_else(|| format!("Malformed data file; name missing"))?,
        points,
      ));
      name = None;
      points = Vec::new();
    } else {
      points.push(parse_point(line)?);
    }
  }
  if name.is_some() && points.len() > 0 {
    result.push(Scanner::create(name.unwrap(), points));
  }

  Ok(result)
}
