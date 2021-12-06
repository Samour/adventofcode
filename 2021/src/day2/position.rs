// ----------- Movement -----------

pub enum MovementDirection {
  Forward,
  Up,
  Down,
}

pub struct Movement {
  pub direction: MovementDirection,
  pub distance: i32,
}

// ----------- Position -----------

pub struct PositionMutation {
  length: i32,
  depth: i32,
  aim: i32,
}

pub struct Position {
  movement_strategy: Box<dyn MovementStrategy>,
  pub length: i32,
  pub depth: i32,
  aim: i32,
}

impl Position {
  pub fn new(movement_strategy: Box<dyn MovementStrategy>) -> Position {
    Position {
      movement_strategy: movement_strategy,
      length: 0,
      depth: 0,
      aim: 0,
    }
  }

  pub fn apply_transition(&mut self, movement: Movement) {
    let mutation = self.movement_strategy.apply_movement(self, movement);
    self.length += mutation.length;
    self.depth += mutation.depth;
    self.aim += mutation.aim;
  }

  pub fn compute_mult(&self) -> i32 {
    self.depth * self.length
  }
}

// ----------- MovementStrategy -----------

pub trait MovementStrategy {
  fn apply_movement(&self, position: &Position, movement: Movement) -> PositionMutation;
}

// 1. SimpleMovementStrategy
pub struct SimpleMovementStrategy;

impl SimpleMovementStrategy {
  pub fn new() -> SimpleMovementStrategy {
    SimpleMovementStrategy
  }
}

impl MovementStrategy for SimpleMovementStrategy {
  fn apply_movement(&self, _position: &Position, movement: Movement) -> PositionMutation {
    let mut mutation = PositionMutation {
      length: 0,
      depth: 0,
      aim: 0,
    };
    match movement.direction {
      MovementDirection::Forward => mutation.length = movement.distance,
      MovementDirection::Up => mutation.depth = -movement.distance,
      MovementDirection::Down => mutation.depth = movement.distance,
    }

    mutation
  }
}

// 2. Aiming movement strategy
pub struct AimingMovementStrategy;

impl AimingMovementStrategy {
  pub fn new() -> AimingMovementStrategy {
    AimingMovementStrategy
  }
}

impl MovementStrategy for AimingMovementStrategy {
  fn apply_movement(&self, position: &Position, movement: Movement) -> PositionMutation {
    let mut mutation = PositionMutation {
      length: 0,
      depth: 0,
      aim: 0,
    };
    match movement.direction {
      MovementDirection::Forward => {
        mutation.length = movement.distance;
        mutation.depth = movement.distance * position.aim;
      }
      MovementDirection::Up => mutation.aim = -movement.distance,
      MovementDirection::Down => mutation.aim = movement.distance,
    }

    mutation
  }
}
