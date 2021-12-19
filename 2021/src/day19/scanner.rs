use ndarray::Array1;
use std::collections::HashMap;

use crate::day19::space::distance;

const MAX_DISTANCE: f64 = 1_000f64;

fn add_point(
  map: &mut HashMap<i32, Vec<(Array1<i32>, Array1<i32>)>>,
  length: i32,
  point_pair: (Array1<i32>, Array1<i32>),
) {
  let mut pairs = map.remove(&length).unwrap_or_else(|| Vec::new());
  map.insert(length, pairs);
}

fn measure_lengths(points: &Vec<Array1<i32>>) -> HashMap<i32, Vec<(Array1<i32>, Array1<i32>)>> {
  let mut result: HashMap<i32, Vec<(Array1<i32>, Array1<i32>)>> = HashMap::new();
  for i in 0..points.len() {
    for j in 1..points.len() {
      let d = distance(&points[i], &points[j]);
      if d <= MAX_DISTANCE {
        add_point(
          &mut result,
          // Need to round due to issues in using floating-point as map key
          // Hopefully does not cause issues :(
          d as i32,
          (points[i].clone(), points[j].clone()),
        );
      }
    }
  }

  result
}

pub struct Scanner {
  pub name: String,
  pub detected: Vec<Array1<i32>>,
  pub lengths: HashMap<i32, Vec<(Array1<i32>, Array1<i32>)>>,
}

impl Scanner {
  pub fn create(name: String, detected: Vec<Array1<i32>>) -> Scanner {
    let lengths = measure_lengths(&detected);
    Scanner {
      name,
      detected,
      lengths,
    }
  }
}
