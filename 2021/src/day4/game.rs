// Structures:
// Board (initial state)
// - Rows
// - - Cells
// During "gameplay", Board will need some more (dynamic) properties
// Board contains Cell[]
// Each cell has following properties:
// - Value (i32)
// - Marked (boolean)
// - WinGroups[]
// A WinGroup is a grouping of Cells which, if all are Marked, that Board has won
// For purposes of modelling, we will represent the WinGroup as the following structure:
// - MarkedCount (i32)
// - WinThreshold (i32)
// Thus, the heirarchy is Board -> Cell -> WinGroup
// Once any WinGroup MarkedCount == WinThreshold, that board has won the game

use std::collections::HashSet;

pub struct WinGroup {
  cell_numbers: HashSet<i32>,
  marked_count: usize,
}

impl WinGroup {
  pub fn new(cell_numbers: HashSet<i32>) -> WinGroup {
    WinGroup {
      cell_numbers,
      marked_count: 0,
    }
  }
}

pub struct Cell {
  number: i32,
  marked: bool,
}

impl Cell {
  pub fn new(number: i32) -> Cell {
    Cell {
      number,
      marked: false,
    }
  }
}

pub struct BingoBoard {
  cells: Vec<Cell>,
  win_groups: Vec<WinGroup>,
}

impl BingoBoard {
  pub fn new(cells: Vec<Cell>, win_groups: Vec<WinGroup>) -> BingoBoard {
    BingoBoard { cells, win_groups }
  }

  pub fn mark_cell(&mut self, number: i32) -> bool {
    let mut was_marked = false;
    for cell in &mut self.cells {
      if cell.number == number {
        was_marked = cell.marked;
        cell.marked = true;
      }
    }

    let mut has_won = false;
    for win_group in &mut self.win_groups {
      if win_group.cell_numbers.contains(&number) {
        if !was_marked {
          win_group.marked_count += 1;
        }
        has_won |= win_group.marked_count == win_group.cell_numbers.len();
      }
    }

    has_won
  }

  pub fn score(&self, mult: i32) -> i32 {
    let mut unmarked_sum: i32 = 0;
    for cell in &self.cells {
      if !cell.marked {
        unmarked_sum += cell.number;
      }
    }

    unmarked_sum * mult
  }
}
