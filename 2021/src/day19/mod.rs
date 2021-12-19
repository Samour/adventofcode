use serde::Deserialize;

use crate::config::{Context, ContextFactory};
use crate::writer::Writer;

mod parser;
mod scanner;
mod solver;
mod space;

use parser::parse;
use scanner::Scanner;

#[derive(Deserialize)]
struct Config {
  scanner_file: String,
}

fn analyse(scanners: Vec<Scanner>) {
  println!("No. of scanners: {}", scanners.len());
  for scanner in scanners {
    println!("{} - no of lines = {}", scanner.name, scanner.lengths.len());
  }
}

pub fn main(factory: ContextFactory, writer: Writer) -> Result<String, String> {
  let context: Context<Config> = factory.create()?;
  let raw_data = context.load_data(&context.config.scanner_file)?;
  let scanners = parse(&raw_data)?;
  analyse(scanners);

  Err(format!("TODO complete"))
}
