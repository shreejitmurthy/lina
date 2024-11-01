use std::fmt::Write;

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
        let mut result = String::new(); // Create an empty `String` to collect the output

        for row in &self.data {
            write!(result, "|").unwrap(); // Start each row with a `|`
            
            for elem in row {
                write!(result, " {:<2}", elem).unwrap(); // Add each element with padding
            }
    
            writeln!(result, "|").unwrap(); // End each row with a `|`
        }
    
        result // Return the final collected string
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
