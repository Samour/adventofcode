use crate::config::ContextFactory;
use serde::Deserialize;

const BASE_10: u32 = 10;
const CODE_LENGTH: u32 = 6;

#[derive(Deserialize)]
struct Config {
  lower: u32,
  upper: u32,
  double_digits_isolated: bool,
  debug: Option<bool>,
}

fn get_digit(value: u32, pos: u32) -> u32 {
  let value = value / BASE_10.pow(pos);

  value % BASE_10
}

fn replace_digits_from(value: u32, pos: u32, n: u32) -> u32 {
  let sub_val = value % BASE_10.pow(pos + 1);
  let mut add_val = 0;
  for _ in 0..(pos + 1) {
    add_val *= BASE_10;
    add_val += n;
  }

  value - sub_val + add_val
}

fn adjust_for_ad(value: u32) -> u32 {
  let mut last_dig = get_digit(value, CODE_LENGTH - 1);
  for i in (0..(CODE_LENGTH - 1)).rev() {
    let this_dig = get_digit(value, i);
    if this_dig < last_dig {
      return replace_digits_from(value, i, last_dig);
    }
    
    last_dig = this_dig;
  }

  value
}

fn has_double_dig(value: u32) -> bool {
  let mut last_dig = get_digit(value, 0);
  for i in 1..CODE_LENGTH {
    let this_dig = get_digit(value, i);
    if this_dig == last_dig {
      return true;
    }

    last_dig = this_dig;
  }

  false
}

fn has_isolated_double_dig(value: u32) -> bool {
  let mut last_dig = get_digit(value, 0);
  let mut run_length = 1;
  for i in 1..CODE_LENGTH {
    let this_dig = get_digit(value, i);
    if this_dig == last_dig {
      run_length += 1;
    } else if run_length == 2 {
      return true;
    } else {
      run_length = 1;
    }

    last_dig = this_dig;
  }

  run_length == 2
}

fn increment(mut value: u32, cap: u32, has_valid_repitition: fn(value: u32) -> bool) -> u32 {
  value += 1;
  if value > cap {
    return value;
  }

  value = adjust_for_ad(value);

  if has_valid_repitition(value) {
    value
  } else {
    increment(value, cap, has_valid_repitition)
  }
}

fn repitition_requirement(require_isolated: bool) -> fn(value: u32) -> bool {
  if require_isolated {
    has_isolated_double_dig
  } else {
    has_double_dig
  }
}

pub fn main(context_factory: ContextFactory) -> Result<(), String> {
  let context = context_factory.create::<Config>()?;
  let debug = context.config.debug.unwrap_or(false);
  let has_valid_repitition = repitition_requirement(context.config.double_digits_isolated);

  let mut count = 0;
  let mut value = context.config.lower - 1;
  loop {
    value = increment(value, context.config.upper, has_valid_repitition);
    if value > context.config.upper {
      break;
    }
    if debug {
      println!("DEBUG: {}", value);
    }

    count += 1;
  }

  println!("Matching values: {}", count);

  Ok(())
}
