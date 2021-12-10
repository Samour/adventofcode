use std::vec::Vec;

use crate::day3::reducer::ValuesReducer;

fn filter_by_mask_idx(
  reducer: Box<dyn ValuesReducer>,
  values: &Vec<String>,
  index: usize,
) -> Option<String> {
  let mask = reducer.reduce(values);
  let chr = mask.chars().nth(index)?;
  let filtered: Vec<String> = values
    .iter()
    .filter(|v| match v.chars().nth(index) {
      Some(c) => chr == c,
      _ => false,
    })
    .cloned()
    .collect();

  match filtered.len() {
    0 => None,
    1 => Some(filtered[0].clone()),
    _ => filter_by_mask_idx(reducer, &filtered, index + 1),
  }
}

pub fn filter_by_mask(reducer: Box<dyn ValuesReducer>, values: &Vec<String>) -> Option<String> {
  filter_by_mask_idx(reducer, values, 0)
}
