use crate::lexer::*;

#[derive(Debug)]
pub struct ParsedCommand {
    command: String,
    matrix_name: String,
    args: String,
}

pub fn parser(tokens: Vec<Token>) -> Result<ParsedCommand, ParseError> {
    if tokens.len() < 2 {
        return Err(ParseError::InvalidFormat);
    }

    let command = if let Token::Command(cmd) = &tokens[0] {
        cmd.clone()
    } else {
        return Err(ParseError::InvalidFormat);
    };

    let matrix_name = if let Token::MatrixName(name) = &tokens[1] {
        if name.len() == 1 && name.chars().all(|c| c.is_ascii_uppercase()) {
            name.clone()
        } else {
            return Err(ParseError::InvalidFormat);
        }
    } else {
        return Err(ParseError::InvalidFormat);
    };

    let args = if tokens.len() > 2 {
        if let Token::Args(arg) = &tokens[2] {
            arg.clone()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
 
    Ok(ParsedCommand {
        command,
        matrix_name,
        args,
    })
}

pub fn parse_input(input: &str) -> Result<ParsedCommand, ParseError> {
    let tokens = lexer(input)?;
    parser(tokens)
}