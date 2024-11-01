// For calculations and parsed command interpretations.

use std::collections::HashMap;
use crate::parser::*;
use crate::matrix::*;

pub struct Calculator {
    // { MatrixName, Matrix }
    pub memory: HashMap<String, Matrix>
}
