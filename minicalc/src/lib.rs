use std::{env, process};

#[derive(Debug)]
pub struct Config {
    pub num1: i32,
    pub operate: String,
    pub num2: i32,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let num1 = match args.next() {
            Some(arg) => arg.trim().parse().unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1)
            }),
            None => return Err("Didn't get the num1"),
        };

        let operate = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the operate"),
        };

        let num2 = match args.next() {
            Some(arg) => arg.trim().parse().unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1)
            }),
            None => return Err("Didn't get the num2"),
        };

        Ok(Self {
            num1,
            operate,
            num2,
        })
    }
}

pub fn calculate(config: Config) -> Result<i32, &'static str> {
    match &config.operate[..] {
        "+" => Ok(config.num1 + config.num2),
        "-" => Ok(config.num1 - config.num2),
        "x" | "*" | "X" => Ok(config.num1 * config.num2),
        "/" => Ok(config.num1 / config.num2),
        "%" => Ok(config.num1 % config.num2),
        _ => return Err("Calculate error"),
    }
}
