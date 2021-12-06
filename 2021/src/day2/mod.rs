mod position;

use position::{Movement, MovementDirection, MovementStrategy, Position, SimpleMovementStrategy};

use crate::config;

use serde::Deserialize;
use std::vec::Vec;

#[derive(Deserialize)]
struct Config {
  directions_file: String,
}

fn parse_movement(raw: &str) -> Option<Movement> {
  let parts: Vec<&str> = raw.split(" ").collect();
  if parts.len() < 2 {
    return None;
  }

  let direction = match parts[0] {
    "forward" => MovementDirection::Forward,
    "up" => MovementDirection::Up,
    "down" => MovementDirection::Down,
    _ => return None,
  };
  let distance: i32 = match parts[1].parse() {
    Ok(i) => i,
    Err(_) => return None,
  };
  Some(Movement {
    direction: direction,
    distance: distance,
  })
}

fn load_movements(raw_movements: Vec<&str>) -> Vec<Movement> {
  raw_movements
    .iter()
    .map(|m| parse_movement(m))
    .filter(|m| m.is_some())
    .map(|m| m.unwrap())
    .collect()
}

fn select_strategy() -> Box<dyn MovementStrategy> {
  Box::new(SimpleMovementStrategy::new())
}

fn compute_motion(movements: Vec<Movement>) {
  let mut position = Position::new(select_strategy());
  for movement in movements {
    position.apply_transition(movement);
  }

  let mult = position.compute_mult();
  println!(
    "Position is length={} depth={}; mult is {}",
    position.length, position.depth, mult
  );
}

pub fn main(factory: config::ContextFactory) -> Result<(), String> {
  let context: config::Context<Config> = factory.create()?;
  let file_contents = context.load_data(&context.config.directions_file)?;
  let raw_movements: Vec<&str> = file_contents.split("\n").collect();
  let movements = load_movements(raw_movements);
  compute_motion(movements);

  Ok(())
}
