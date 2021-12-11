use serde::Deserialize;

use crate::config::{Context, ContextFactory};

mod game;
mod parse;

use game::BingoBoard;
use parse::{parse_game, BingoGame};

#[derive(Deserialize)]
struct Config {
  game_file: String,
  find_winner: String,
}

fn find_first_winner(numbers: Vec<i32>, mut boards: Vec<BingoBoard>) {
  for number in numbers {
    for board in &mut boards {
      if board.mark_cell(number) {
        println!("Winner found! Score: {}", board.score(number));
        return;
      }
    }
  }

  println!("No winner found");
}

fn find_last_winner(numbers: Vec<i32>, mut boards: Vec<BingoBoard>) {
  for number in numbers {
    let boards_len = boards.len();
    let mut not_won_boards: Vec<BingoBoard> = Vec::new();
    for mut board in boards {
      if board.mark_cell(number) {
        if boards_len == 1 {
          println!("Losing board score: {}", board.score(number));
          return;
        }
      } else {
        not_won_boards.push(board);
      }
    }
    boards = not_won_boards;
  }

  match boards.len() {
    0 => println!("No single loser found!"),
    _ => println!("Multiple boards left after all numbers"),
  }
}

fn select_strategy(strategy_name: &str) -> Result<fn(Vec<i32>, Vec<BingoBoard>) -> (), String> {
  match strategy_name {
    "first" => Ok(find_first_winner),
    "last" => Ok(find_last_winner),
    _ => Err(String::from("Strategy name not recognized")),
  }
}

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.game_file)?;
  let BingoGame { numbers, boards } = parse_game(raw_data);

  select_strategy(&context.config.find_winner)?(numbers, boards);

  Ok(())
}
