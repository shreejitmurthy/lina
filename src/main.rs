mod matrix;
mod lexer;
mod parser;
mod calculator;

use parser::*;
use calculator::*;

use std::io;
use std::io::Write;

use std::process::exit;

fn main() {
    let mut calc = Calculator::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        if input.trim() == "exit" || input.trim() == "ex" {
            println!("Exiting...");
            exit(0);
        }
    
        match parse_input(input.as_str()) {
            Ok(parsed_command) => {
                let result = calc.interpret(parsed_command);
                println!("{}", result)
            }
            Err(e) => {
                println!("Error parsing input: {:?}", e);
            }
        }
    }
}
