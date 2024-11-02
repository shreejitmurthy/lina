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
            "define"   | "def"  => self.define(parsed_command.matrix_name, parsed_command.args),
            "echo"     | "show" => self.show(parsed_command.matrix_name),
            // TODO: These
            "populate" | "fill" => self.fill(parsed_command.matrix_name, parsed_command.args),
            "add"      | "sum"  => self.sum(parsed_command.matrix_name, parsed_command.args),
            _ => "".to_string()
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

    fn show(&self, matrix_name: String) -> String {
        if let Some(matrix) = self.memory.get(&matrix_name) {
            matrix.to_string()
        } else {
            format!("Matrix '{}' not found in memory.", matrix_name)
        }
    }

    fn fill(&mut self, matrix_name: String, args: String) -> String {
        if let Some(matrix) = self.memory.get_mut(&matrix_name) {
            match parse_matrix_data(args.as_str()) {
                Ok(parsed_data) => { 
                    matrix.data = parsed_data; 
                    format!("Populated Matrix {} with data.", matrix_name)
                }
                Err(e) => { format!("Error parsing matrix data: {}", e) }
            }
        } else {
            format!("Matrix '{}' not found in memory.", matrix_name)
        }
    }

    fn sum(&self, matrix_name: String, args: String) -> String {
        let m1 = self.memory.get(&matrix_name).unwrap().clone();
        let m2 = self.memory.get(&args).unwrap().clone();
        let result = sum_matrices(m1, m2);
        result.unwrap().to_string()
    }
}
