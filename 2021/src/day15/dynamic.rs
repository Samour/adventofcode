use std::collections::HashMap;

struct DynamicRisk {
  specific_risk: HashMap<(i32, i32), i32>,
  path_risk: HashMap<(i32, i32), i32>,
}

impl DynamicRisk {
  fn new(specific_risk: HashMap<(i32, i32), i32>) -> DynamicRisk {
    let mut path_risk: HashMap<(i32, i32), i32> = HashMap::new();
    path_risk.insert((0, 0), 0);
    DynamicRisk {
      specific_risk,
      path_risk,
    }
  }

  fn compute_path_cost(&mut self, x: i32, y: i32) -> Result<i32, String> {
    match self.path_risk.get(&(x, y)) {
      Some(&r) => return Ok(r),
      None => {}
    }
    let mut cost: i32 = i32::MAX;
    for n in vec![(x - 1, y), (x, y - 1)] {
      if !self.specific_risk.contains_key(&n) {
        continue;
      }
      let n_cost = self.compute_path_cost(n.0, n.1)?;
      if n_cost < cost {
        cost = n_cost;
      }
    }

    cost += self
      .specific_risk
      .get(&(x, y))
      .ok_or_else(|| format!("Coordinates are not within risk map"))?;
    self.path_risk.insert((x, y), cost);
    Ok(cost)
  }
}

pub fn compute_risk(specific_risk: HashMap<(i32, i32), i32>) -> Result<i32, String> {
  let max_x = specific_risk.keys().map(|&(x, _)| x).max().unwrap_or(0);
  let max_y = specific_risk.keys().map(|&(_, y)| y).max().unwrap_or(0);
  DynamicRisk::new(specific_risk).compute_path_cost(max_x, max_y)
}
