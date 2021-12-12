#![allow(warnings)] 

mod args;
mod config;
mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod implementations;
mod test_harness;
mod writer;

fn execute_single(config_fname: String) -> Result<(), String> {
    implementations::execute(config_fname, writer::Writer::StdoutWriter)
}

fn main() -> Result<(), String> {
    let mut parser = args::ArgsParser::new();
    parser.arg_str("config");
    parser.arg_str("tests");
    parser.parse();

    let tests = parser.get_flag("tests");
    let config = parser.get_flag("config");
    if tests.is_some() {
        test_harness::execute_tests(tests.unwrap())
    } else if config.is_some() {
        execute_single(config.unwrap())
    } else {
        Err(String::from("One of --tests or --config must be specified"))
    }
}
