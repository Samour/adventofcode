use serde::Deserialize;

use crate::config::{Context, ContextFactory};

mod game;
mod parse;

use parse::{parse_game, BingoGame};

#[derive(Deserialize)]
struct Config {
  game_file: String,
}

pub fn main(factory: ContextFactory) -> Result<(), String> {
  let context: Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.game_file)?;
  let BingoGame {
    numbers,
    mut boards,
  } = parse_game(raw_data);

  for number in numbers {
    for board in &mut boards {
      if board.mark_cell(number) {
        println!("Winner found! Score: {}", board.score(number));
        return Ok(());
      }
    }
  }

  println!("No winner found");

  Ok(())
}
