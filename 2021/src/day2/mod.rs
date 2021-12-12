mod position;

use position::{
  AimingMovementStrategy, Movement, MovementDirection, MovementStrategy, Position,
  SimpleMovementStrategy,
};

use crate::config;
use crate::writer::Writer;

use serde::Deserialize;
use std::vec::Vec;

#[derive(Deserialize)]
struct Config {
  movement_strategy: String,
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

fn select_strategy(strategy_name: &str) -> Result<Box<dyn MovementStrategy>, String> {
  match strategy_name {
    "SimpleMovementStrategy" => Ok(Box::new(SimpleMovementStrategy::new())),
    "AimingMovementStrategy" => Ok(Box::new(AimingMovementStrategy::new())),
    _ => Err(String::from("Unrecognized movement strategy")),
  }
}

fn compute_motion(
  movement_strategy: Box<dyn MovementStrategy>,
  movements: Vec<Movement>,
  writer: Writer,
) -> i64 {
  let mut position = Position::new(movement_strategy);
  for movement in movements {
    position.apply_transition(movement);
  }

  let mult = position.compute_mult();
  writer.write(|| {
    format!(
      "Position is length={} depth={}; mult is {}",
      position.length, position.depth, mult
    )
  });

  mult as i64
}

pub fn main(factory: config::ContextFactory, writer: Writer) -> Result<i64, String> {
  let context: config::Context<Config> = factory.create()?;
  let file_contents = context.load_data(&context.config.directions_file)?;
  let raw_movements: Vec<&str> = file_contents.split("\n").collect();
  let movements = load_movements(raw_movements);
  let movement_strategy = select_strategy(&context.config.movement_strategy)?;
  Ok(compute_motion(movement_strategy, movements, writer))
}
