use std::cell::RefCell;
use std::rc::Rc;

pub enum SnailFishNumberContent {
  Regular(i32),
  Pair(Rc<RefCell<SnailFishNumber>>, Rc<RefCell<SnailFishNumber>>),
}

impl SnailFishNumberContent {
  pub fn is_pair(&self) -> bool {
    match self {
      SnailFishNumberContent::Pair(_, _) => true,
      _ => false,
    }
  }

  pub fn regular_or_empty(&self) -> Option<i32> {
    match self {
      SnailFishNumberContent::Regular(v) => Some(*v),
      _ => None,
    }
  }

  pub fn pair_or_empty(
    &self,
  ) -> Option<(Rc<RefCell<SnailFishNumber>>, Rc<RefCell<SnailFishNumber>>)> {
    match self {
      SnailFishNumberContent::Pair(left, right) => Some((Rc::clone(left), Rc::clone(right))),
      _ => None,
    }
  }

  pub fn magnitude(&self) -> i32 {
    match self {
      SnailFishNumberContent::Regular(i) => *i,
      SnailFishNumberContent::Pair(left, right) => {
        3 * left.borrow().content.magnitude() + 2 * right.borrow().content.magnitude()
      }
    }
  }
}

pub struct SnailFishNumber {
  pub content: SnailFishNumberContent,
}

impl SnailFishNumber {
  pub fn new(content: SnailFishNumberContent) -> SnailFishNumber {
    SnailFishNumber { content }
  }

  pub fn render(&self) -> String {
    match &self.content {
      SnailFishNumberContent::Regular(i) => format!("{}", i),
      SnailFishNumberContent::Pair(left, right) => {
        format!("[{},{}]", left.borrow().render(), right.borrow().render())
      }
    }
  }
}

#[derive(Clone, Copy)]
pub enum Direction {
  Left,
  Right,
}

impl Direction {
  pub fn is_left(&self) -> bool {
    match self {
      Direction::Left => true,
      Direction::Right => false,
    }
  }
}

pub fn plus(
  one: Rc<RefCell<SnailFishNumber>>,
  two: Rc<RefCell<SnailFishNumber>>,
) -> Rc<RefCell<SnailFishNumber>> {
  Rc::new(RefCell::new(SnailFishNumber::new(
    SnailFishNumberContent::Pair(one, two),
  )))
}
