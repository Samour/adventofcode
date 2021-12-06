use serde::Deserialize;

mod args;
mod config;
mod day1;
mod day2;
mod day3;

#[derive(Deserialize)]
struct Config {
    implementation: String,
}

fn select_impl(name: &str) -> Option<fn(factory: config::ContextFactory) -> Result<(), String>> {
    match name {
        "day1" => Some(day1::main),
        "day2" => Some(day2::main),
        "day3" => Some(day3::main),
        _ => None,
    }
}

fn main() -> Result<(), String> {
    let mut parser = args::ArgsParser::new();
    parser.arg_str("config");
    parser.parse();

    let config = parser.get_flag("config");
    if config.is_none() {
        return Err(String::from("Config file not specified"));
    }

    let context_factory = config::ContextFactory::new(config.unwrap());
    let config = context_factory.create::<Config>()?.config;

    let problem_impl = select_impl(&config.implementation);

    if problem_impl.is_none() {
        return Err(format!(
            "No implementation found for {}",
            &config.implementation
        ));
    }

    problem_impl.unwrap()(context_factory)
}
