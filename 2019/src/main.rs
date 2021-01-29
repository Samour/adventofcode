use serde::Deserialize;

mod args;
mod config;
mod intcode;
mod prob1;
mod prob2;
mod prob3;
mod prob4;
mod prob5;

#[derive(Deserialize)]
struct Config {
    implementation: String,
}

fn select_impl(name: &str) -> Option<fn(factory: config::ContextFactory) -> Result<(), String>> {
    match name {
        "prob1" => Some(prob1::main),
        "prob2" => Some(prob2::main),
        "prob3" => Some(prob3::main),
        "prob4" => Some(prob4::main),
        "prob5" => Some(prob5::main),
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
