use std::collections::{HashMap, HashSet};

use crate::day4::game::{BingoBoard, Cell, WinGroup};

pub struct BingoGame {
  pub numbers: Vec<i32>,
  pub boards: Vec<BingoBoard>,
}

struct BoardSpec {
  rows: HashMap<usize, HashSet<i32>>,
  columns: HashMap<usize, HashSet<i32>>,
}

fn push_into_map(map: &mut HashMap<usize, HashSet<i32>>, key: usize, number: i32) {
  if map.contains_key(&key) {
    map.get_mut(&key).unwrap().insert(number);
  } else {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(number);
    map.insert(key, set);
  }
}

impl BoardSpec {
  fn new() -> BoardSpec {
    BoardSpec {
      rows: HashMap::new(),
      columns: HashMap::new(),
    }
  }

  fn push_cell(&mut self, row: usize, column: usize, number: i32) {
    push_into_map(&mut self.rows, row, number);
    push_into_map(&mut self.columns, column, number);
  }

  fn create_board(&self) -> BingoBoard {
    let cells: Vec<Cell> = self
      .rows
      .values()
      .flat_map(|v| v.iter())
      .map(|&n| Cell::new(n))
      .collect();
    let mut win_groups: Vec<WinGroup> = Vec::new();
    for group_container in &[&self.rows, &self.columns] {
      for group in group_container.values() {
        win_groups.push(WinGroup::new(group.clone()))
      }
    }

    BingoBoard::new(cells, win_groups)
  }
}

pub fn parse_game(game_raw: String) -> BingoGame {
  let raw_lines: Vec<&str> = game_raw.split("\n").collect();
  let numbers: Vec<i32> = raw_lines[0]
    .split(",")
    .map(|v| v.parse::<i32>())
    .filter(|v| v.is_ok())
    .map(|v| v.unwrap())
    .collect();

  let mut boards: Vec<BingoBoard> = Vec::new();
  let mut board_spec = BoardSpec::new();
  let mut row: usize = 0;
  for line in raw_lines.iter().skip(2) {
    if line.len() == 0 {
      boards.push(board_spec.create_board());
      board_spec = BoardSpec::new();
      row = 0;
    } else {
      let numbers: Vec<i32> = line
        .split(" ")
        .map(|v| v.parse::<i32>())
        .filter(|v| v.is_ok())
        .map(|v| v.unwrap())
        .collect();
      let mut column: usize = 0;
      for number in numbers {
        board_spec.push_cell(row, column, number);
        column += 1;
      }
      row += 1;
    }
  }
  if board_spec.rows.len() > 0 {
    boards.push(board_spec.create_board());
  }

  BingoGame {
    numbers,
    boards,
  }
}
