use minicalc::{calculate, Config};
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    let result = calculate(config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    println!("{}", result)
}
