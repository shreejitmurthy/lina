mod matrix;
mod lexer;
mod parser;
mod calculator;

use parser::*;

// fn main() {
//     let m1 = Matrix::new(2, 3);
//     // let m2 = Matrix::new(3, 2);

//     // let result = multiply_matrices(m1, m2);
//     let result = transpose_matrix(m1);

//     result.print();
// }

fn main() {
    let input = "define A 2x2";
    match parse_input(input) {
        Ok(parsed_command) => {
            println!("{:?}", parsed_command);
        }
        Err(e) => {
            println!("Error parsing input: {:?}", e);
        }
    }
    

}
