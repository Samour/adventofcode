use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::day18::sfn::{SnailFishNumber, SnailFishNumberContent};

enum ParserToken {
  PairOpen,
  Integer(i32),
}

fn tokenise(source: String) -> Result<VecDeque<ParserToken>, String> {
  let mut result: VecDeque<ParserToken> = VecDeque::new();
  let mut in_int = false;
  let mut int_val: i32 = 0;
  for c in source.chars() {
    match c {
      '[' => result.push_back(ParserToken::PairOpen),
      ']' => {
        if in_int {
          result.push_back(ParserToken::Integer(int_val));
          in_int = false;
          int_val = 0;
        }
      }
      ',' => {
        if in_int {
          result.push_back(ParserToken::Integer(int_val));
          in_int = false;
          int_val = 0;
        }
      }
      _ => match String::from(c).parse::<i32>() {
        Ok(i) => {
          in_int = true;
          int_val *= 10;
          int_val += i;
        }
        Err(_) => return Err(format!("Unrecognized character during parsing")),
      },
    }
  }

  Ok(result)
}

struct SnailFishNumberParser {
  source: VecDeque<ParserToken>,
}

impl SnailFishNumberParser {
  fn new(source: VecDeque<ParserToken>) -> SnailFishNumberParser {
    SnailFishNumberParser { source }
  }

  fn parse(&mut self) -> Result<Rc<RefCell<SnailFishNumber>>, String> {
    let token = self
      .source
      .pop_front()
      .ok_or_else(|| format!("Attempted to parse, but input ended unexpectedly"))?;
    match token {
      ParserToken::Integer(i) => Ok(Rc::new(RefCell::new(SnailFishNumber::new(
        SnailFishNumberContent::Regular(i),
      )))),
      ParserToken::PairOpen => {
        let left = self.parse()?;
        let right = self.parse()?;
        Ok(Rc::new(RefCell::new(SnailFishNumber::new(
          SnailFishNumberContent::Pair(left, right),
        ))))
      }
    }
  }
}

pub fn parse(source: String) -> Result<Rc<RefCell<SnailFishNumber>>, String> {
  SnailFishNumberParser::new(tokenise(source)?).parse()
}
