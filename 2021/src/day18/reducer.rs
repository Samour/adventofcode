use std::cell::RefCell;
use std::rc::Rc;

use crate::writer::Writer;

use crate::day18::sfn::{Direction, SnailFishNumber, SnailFishNumberContent};

struct NumberReducer<'a> {
  root_node: Rc<RefCell<SnailFishNumber>>,
  debug: bool,
  writer: &'a Writer,
}

impl NumberReducer<'_> {
  fn new<'a>(
    root_node: Rc<RefCell<SnailFishNumber>>,
    debug: bool,
    writer: &'a Writer,
  ) -> NumberReducer<'a> {
    NumberReducer {
      root_node,
      debug,
      writer,
    }
  }

  fn increment_sibling(
    &self,
    direction: Direction,
    mut walk: Vec<(Direction, Rc<RefCell<SnailFishNumber>>)>,
    increment: i32,
  ) -> Result<(), String> {
    // Walk up path until we reach the lowest node with an alternate branch in the required direction
    let mut node: Option<Rc<RefCell<SnailFishNumber>>> = None;
    loop {
      match walk.pop() {
        Some((d, n)) => {
          if d.is_left() != direction.is_left() {
            node = Some(n);
            break;
          }
        }
        // There is no alternate path
        None => return Ok(()),
      }
    }
    let mut node = node.ok_or_else(|| format!("Tree was corrupted during walk"))?;
    // 1 step in required direction
    node = Rc::clone(&node)
      .borrow()
      .content
      .pair_or_empty()
      .map(|(left, right)| match direction {
        Direction::Left => left,
        Direction::Right => right,
      })
      .ok_or_else(|| format!("Tree was corrupted during walk"))?;
    // Walk all children in opposite direction so that we find the number closest to the original node
    while let SnailFishNumberContent::Pair(left, right) = &Rc::clone(&node).borrow().content {
      node = match direction {
        Direction::Left => Rc::clone(right),
        Direction::Right => Rc::clone(left),
      };
    }
    // Apply increment
    let value = node
      .borrow()
      .content
      .regular_or_empty()
      .ok_or_else(|| format!("Tree was corrupted during walk"))?;
    node.borrow_mut().content = SnailFishNumberContent::Regular(value + increment);

    Ok(())
  }

  fn apply_explosions(
    &self,
    node: Rc<RefCell<SnailFishNumber>>,
    walk: Vec<(Direction, Rc<RefCell<SnailFishNumber>>)>,
  ) -> Result<bool, String> {
    let mut new_content: Option<SnailFishNumberContent> = None;
    match &Rc::clone(&node).borrow().content {
      SnailFishNumberContent::Pair(left_c, right_c) => {
        if walk.len() == 4 {
          // Explode
          let left = Rc::clone(left_c)
            .borrow()
            .content
            .regular_or_empty()
            .ok_or_else(|| format!("Attempted to explode node, but child is not Regular"))?;
          let right = Rc::clone(&right_c)
            .borrow()
            .content
            .regular_or_empty()
            .ok_or_else(|| format!("Attempted to explode node, but child is not Regular"))?;
          self.increment_sibling(Direction::Left, walk.clone(), left);
          self.increment_sibling(Direction::Right, walk.clone(), right);
          new_content = Some(SnailFishNumberContent::Regular(0));
        } else {
          // Reduce at most 1 child
          if self.explode_child(
            Rc::clone(&node),
            Rc::clone(left_c),
            walk.clone(),
            Direction::Left,
          )? {
            return Ok(true);
          }
          if self.explode_child(
            Rc::clone(&node),
            Rc::clone(right_c),
            walk.clone(),
            Direction::Right,
          )? {
            return Ok(true);
          }
        }
      }
      _ => {}
    }

    match new_content {
      Some(c) => {
        node.borrow_mut().content = c;
        Ok(true)
      }
      None => Ok(false),
    }
  }

  fn explode_child(
    &self,
    parent_node: Rc<RefCell<SnailFishNumber>>,
    child_node: Rc<RefCell<SnailFishNumber>>,
    mut walk: Vec<(Direction, Rc<RefCell<SnailFishNumber>>)>,
    direction: Direction,
  ) -> Result<bool, String> {
    walk.push((direction, parent_node));
    self.apply_explosions(child_node, walk)
  }

  fn apply_splits(&self, node: Rc<RefCell<SnailFishNumber>>) -> Result<bool, String> {
    let mut new_content: Option<SnailFishNumberContent> = None;
    match &Rc::clone(&node).borrow().content {
      SnailFishNumberContent::Pair(left_c, right_c) => {
        // Reduce at most 1 child
        if self.apply_splits(Rc::clone(left_c))? {
          return Ok(true);
        }
        if self.apply_splits(Rc::clone(right_c))? {
          return Ok(true);
        }
      }
      SnailFishNumberContent::Regular(v) => {
        let v = *v;
        if v >= 10 {
          // Split
          let left = v / 2;
          let right = v - left;
          new_content = Some(SnailFishNumberContent::Pair(
            Rc::new(RefCell::new(SnailFishNumber::new(
              SnailFishNumberContent::Regular(left),
            ))),
            Rc::new(RefCell::new(SnailFishNumber::new(
              SnailFishNumberContent::Regular(right),
            ))),
          ));
        }
      }
    }

    match new_content {
      Some(c) => {
        node.borrow_mut().content = c;
        Ok(true)
      }
      None => Ok(false),
    }
  }

  fn apply_reduce(&self) -> Result<bool, String> {
    if self.apply_explosions(Rc::clone(&self.root_node), Vec::new())? {
      return Ok(true);
    }
    self.apply_splits(Rc::clone(&self.root_node))
  }

  fn reduce(&self) -> Result<(), String> {
    while self.apply_reduce()? {
      if self.debug {
        self
          .writer
          .write(|| format!(" -> {}", self.root_node.borrow().render()));
      }
    }

    Ok(())
  }
}

pub fn reduce(
  number: Rc<RefCell<SnailFishNumber>>,
  debug: bool,
  writer: &Writer,
) -> Result<(), String> {
  NumberReducer::new(number, debug, writer).reduce()
}
