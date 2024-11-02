use std::fmt::Write;
use regex::Regex;


#[derive(Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0.0; cols]; rows];
        Matrix { rows, cols, data }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for row in &self.data {
            write!(result, "|").unwrap();
            
            for elem in row {
                write!(result, " {:<2}", elem).unwrap();
            }
    
            writeln!(result, "|").unwrap();
        }
    
        result
    }

    pub fn print(&self) {
        for row in &self.data {
            print!("|");
            for elem in row {
                print!(" {:<2}", elem);
            }
            println!("|");
        }
    }
}


/* C-style functions for matrix operations. 
 * This was done because the aim is for operations like this: 'add MatA MatB'.
 * This design will benefit from C-style functions, I believe.
 */

pub fn sum_matrices(m1: Matrix, m2: Matrix) -> Result<Matrix, &'static str> {
    if m1.cols != m2.cols || m1.rows != m2.rows {
        return Err("Matrices must have the same dimensions for addition.");
    }

    let mut result = Matrix::new(m1.rows, m2.cols);
    for i in 0..m1.rows {
        for j in 0..m2.cols {
            result.data[i][j] = m1.data[i][j] + m2.data[i][j];
        }
    }
    Ok(result)
}

pub fn subtract_matrices(m1: Matrix, m2: Matrix) -> Result<Matrix, &'static str> {
    if m1.cols != m2.cols || m1.rows != m2.rows {
        return Err("Matrices must have the same dimensions for subtraction.");
    }

    let mut result = Matrix::new(m1.rows, m2.cols);
    for i in 0..m1.rows {
        for j in 0..m2.cols {
            result.data[i][j] = m1.data[i][j] - m2.data[i][j];
        }
    }
    Ok(result)
}

pub fn multiply_matrices(m1: Matrix, m2: Matrix) -> Result<Matrix, &'static str> {
    if m1.cols != m2.rows {
        return Err("Matrices must have opposite dimensions for multiplication.");
    }

    let mut result = Matrix::new(m1.rows, m2.cols);
    for i in 0..m1.rows {
        for j in 0..m2.cols {
            let mut sum = 0.0;
            for k in 0..m1.cols {
                sum += m1.data[i][k] * m2.data[k][j];
            }
            result.data[i][j] = sum;
        }
    }

    Ok(result)

}

pub fn scale_matrix(m: Matrix, s: f64) -> Matrix {
    let mut result = Matrix::new(m.rows, m.cols);
    for i in 0..m.rows {
        for j in 0..m.cols {
            result.data[i][j] = s * m.data[i][j];
        }
    }
    result
}

pub fn transpose_matrix(m: Matrix) -> Matrix {
    let mut result = Matrix::new(m.cols, m.rows);
    for i in 0..m.rows {
        for j in 0..m.cols {
            result.data[j][i] = m.data[i][j];
        }
    }
    result
}

// TODO: Some implementation of this.
pub fn invert_matrix(m: Matrix) -> Result<Matrix, &'static str> {
    if m.rows != m.cols {
        return Err("Matrix must be square (m == n) for inversion.")
    }

    let result = Matrix::new(m.rows, m.cols);
    Ok(result)
}

// Needs to be a string to extract the Regex value 
pub fn parse_matrix_data(input: &str) -> Result<Vec<Vec<f64>>, String> {
    let row_pattern = Regex::new(r"\[([^\[\]]*)\]").map_err(|e| e.to_string())?;
    let number_pattern = Regex::new(r"-?\d+(\.\d+)?").map_err(|e| e.to_string())?;

    let mut matrix_data = Vec::new();

    for row_caps in row_pattern.captures_iter(input) {
        let row_str = &row_caps[1];

        let mut row_data = Vec::new();
        for num_caps in number_pattern.captures_iter(row_str) {
            let num_str = &num_caps[0];
            let num: f64 = num_str.parse().map_err(|e: std::num::ParseFloatError| e.to_string().to_string())?;
            row_data.push(num);
        }

        matrix_data.push(row_data);
    }

    let row_length = matrix_data.first().map_or(0, |row| row.len());
    if matrix_data.iter().any(|row| row.len() != row_length) {
        return Err("Rows have inconsistent lengths.".to_string());
    }

    Ok(matrix_data)
}
