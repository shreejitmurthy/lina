// Simplified Lexer Implementation

#[derive(Debug, PartialEq)]
pub enum Token {
    Command(String),
    MatrixName(String),
    Args(String)
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidFormat,
    MissingMatrix,
    MissingCommand
}

pub fn lexer(input: &str) -> Result<Vec<Token>, ParseError> {
    let segments: Vec<&str> = input.trim().split_whitespace().collect();
    if segments.is_empty() {
        return Err(ParseError::MissingCommand);
    }

    // Don't do .get, gives the Option instead of the string we want.
    let command = segments[0].to_string();
    let matrix_name = segments.get(1)
        .ok_or(ParseError::MissingMatrix)?
        .to_string();

    // Combine the remaining parts for args or just new string if none.
    let args = if segments.len() > 2 {
        segments[2..].join(" ")
    } else {
        String::new()
    };

    let mut tokens = Vec::new();
    tokens.push(Token::Command(command));
    tokens.push(Token::MatrixName(matrix_name));
    tokens.push(Token::Args(args));

    Ok(tokens)
}
