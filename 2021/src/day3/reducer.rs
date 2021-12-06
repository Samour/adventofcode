use std::collections::HashMap;
use std::vec::Vec;

// ---------- ValuesReducer trait ----------

pub trait ValuesReducer {
  fn reduce(&self, values: &Vec<String>) -> String;
}

// ---------- MostCommonBitReducer ----------

struct CommonBitCounter {
  count0: i32,
  count1: i32,
}

impl CommonBitCounter {
  fn new() -> CommonBitCounter {
    CommonBitCounter {
      count0: 0,
      count1: 0,
    }
  }
}

pub struct MostCommonBitReducer;

impl MostCommonBitReducer {
  pub fn new() -> MostCommonBitReducer {
    MostCommonBitReducer
  }
}

impl ValuesReducer for MostCommonBitReducer {
  fn reduce(&self, values: &Vec<String>) -> String {
    let mut counts: HashMap<usize, CommonBitCounter> = HashMap::new();
    for i in 0..values.len() {
      let mut j: usize = 0;
      for c in values[i].chars() {
        let mut position_counts = counts.remove(&j).unwrap_or(CommonBitCounter::new());
        match c {
          '1' => position_counts.count1 += 1,
          '0' => position_counts.count0 += 1,
          _ => {}
        }
        counts.insert(j, position_counts);
        j += 1;
      }
    }

    let mut result = String::new();
    let mut i: usize = 0;
    while counts.contains_key(&i) {
      let position_counts = counts.remove(&i).unwrap_or(CommonBitCounter::new());
      if position_counts.count1 > position_counts.count0 {
        result.push('1');
      } else {
        result.push('0');
      }
      i += 1;
    }

    result
  }
}

// ---------- BitInvertedReducer ----------

pub struct BitInvertedReducer {
  underlying_reducer: Box<dyn ValuesReducer>,
}

impl BitInvertedReducer {
  pub fn new(underlying_reducer: Box<dyn ValuesReducer>) -> BitInvertedReducer {
    BitInvertedReducer {
      underlying_reducer: underlying_reducer,
    }
  }
}

impl ValuesReducer for BitInvertedReducer {
  fn reduce(&self, values: &Vec<String>) -> String {
    let underlying_result = self.underlying_reducer.reduce(values);
    let mut result = String::new();
    for c in underlying_result.chars() {
      match c {
        '1' => result.push('0'),
        _ => result.push('1'),
      }
    }

    result
  }
}

// ---------- LeastCommonBitReducer ----------

pub struct LeastCommonBitReducer {
  underlying_reducer: Box<dyn ValuesReducer>,
}

impl LeastCommonBitReducer {
  pub fn new() -> LeastCommonBitReducer {
    LeastCommonBitReducer {
      underlying_reducer: Box::new(BitInvertedReducer::new(Box::new(
        MostCommonBitReducer::new(),
      ))),
    }
  }
}

impl ValuesReducer for LeastCommonBitReducer {
  fn reduce(&self, values: &Vec<String>) -> String {
    self.underlying_reducer.reduce(values)
  }
}
