pub mod parser;
pub mod utility;

use std::{
    io::{self, Write},
    process::exit,
};
use parser::Expression;
use clap::Command;

pub const VERSION: &str = "0.1.0";

fn main() {
    let mut command = Command::new("amber")
        .version(VERSION)
        .author("mudkip")
        .about("A high-level programming language written in Rust.")
        .subcommand(
            Command::new("shell")
                .about("Launches an interactive shell to try out the language.")
                .alias("repl")
        );

    let help = command.render_help();
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("shell", _)) => launch_shell(),
        _ => println!("{}", help)
    }
}

fn launch_shell() {
    println!(
        "\n{}Amber v{}{}\nType \"exit\" to quit.\n",
        "\x1b[1;32m", // bold red
        VERSION,
        "\x1b[0m" // reset
    );

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let text = buffer.trim().to_lowercase();

        if text.is_empty() {
            continue;
        } else if text == "exit" || text == "quit" {
            exit(0);
        }

        match Expression::new(&text) {
            Some((_, expression)) => {
                if let Some(number) = expression.compute() {
                    println!("{}", number.0);
                } else if expression.is_dividing_by_zero() {
                    println!("Cannot divide by zero!");
                } else {
                    println!("Failed to compute!");
                }
            }
            None => println!("Invalid expression!"),
        }
    }
}