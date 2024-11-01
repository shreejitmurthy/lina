// For calculations and parsed command interpretations.

use std::collections::HashMap;
use crate::parser::*;
use crate::matrix::*;

use regex::Regex;

pub struct Calculator {
    // { MatrixName, Matrix }
    pub memory: HashMap<String, Matrix>
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { memory: HashMap::new() }
    }

    pub fn interpret(&mut self, parsed_command: ParsedCommand) -> String {
        match parsed_command.command.as_str() {
            "define" => self.define(parsed_command.matrix_name, parsed_command.args),
            "show" => todo!(),
            _ => todo!()
        }
    }

    fn define(&mut self, matrix_name: String, args: String) -> String {
        let pattern_regex = Regex::new(r"(?P<m>\d+)x(?P<n>\d+)").unwrap();
        let name = matrix_name.clone();
    
        let mut m: usize = 0;
        let mut n: usize = 0;
    
        if let Some(caps) = pattern_regex.captures(&args) {
            m = caps["m"].parse().unwrap();
            n = caps["n"].parse().unwrap();
        }
    
        let matrix = Matrix::new(m, n);
        self.memory.insert(name.clone(), matrix);
    
        let message = format!("Defined Matrix {} as {}x{} matrix", name, m, n);
        message
    }
}
