use serde::Deserialize;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

mod game;
mod parse;

use game::BingoBoard;
use parse::{parse_game, BingoGame};

#[derive(Deserialize)]
struct Config {
  game_file: String,
  find_winner: String,
}

fn find_first_winner(
  numbers: Vec<i32>,
  mut boards: Vec<BingoBoard>,
  writer: Writer,
) -> Result<i32, String> {
  for number in numbers {
    for board in &mut boards {
      if board.mark_cell(number) {
        let score = board.score(number);
        writer.write(|| format!("Winner found! Score: {}", score));
        return Ok(score);
      }
    }
  }

  writer.write(|| "No winner found");
  Err(format!("No winner found"))
}

fn find_last_winner(
  numbers: Vec<i32>,
  mut boards: Vec<BingoBoard>,
  writer: Writer,
) -> Result<i32, String> {
  for number in numbers {
    let boards_len = boards.len();
    let mut not_won_boards: Vec<BingoBoard> = Vec::new();
    for mut board in boards {
      if board.mark_cell(number) {
        if boards_len == 1 {
          let score = board.score(number);
          writer.write(|| format!("Losing board score: {}", score));
          return Ok(score);
        }
      } else {
        not_won_boards.push(board);
      }
    }
    boards = not_won_boards;
  }

  match boards.len() {
    0 => {
      writer.write(|| "No single loser found!");
      Err(format!("No singler loser found"))
    }
    _ => {
      writer.write(|| "Multiple boards left after all numbers");
      Err(format!("Multiple boards left after all numbers"))
    }
  }
}

fn select_strategy(
  strategy_name: &str,
) -> Result<fn(Vec<i32>, Vec<BingoBoard>, Writer) -> Result<i32, String>, String> {
  match strategy_name {
    "first" => Ok(find_first_winner),
    "last" => Ok(find_last_winner),
    _ => Err(String::from("Strategy name not recognized")),
  }
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<String, String> {
  let context: Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.game_file)?;
  let BingoGame { numbers, boards } = parse_game(raw_data);

  select_strategy(&context.config.find_winner)?(numbers, boards, writer).map(|r| format!("{}", r))
}
