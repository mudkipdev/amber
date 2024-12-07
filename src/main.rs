pub mod parser;
pub mod utility;

use parser::Expression;
use std::{
    io::{self, Write},
    process::exit,
};

pub const VERSION: &str = "0.1.0";

fn main() {
    println!("Amber v{}\nType \"exit\" to quit.\n", VERSION);

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
