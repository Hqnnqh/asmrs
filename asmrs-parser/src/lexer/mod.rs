use crate::lexer::token::Token;
use std::{error::Error, fmt::Display, vec::Vec};

pub mod token;

pub fn tokenize(input: String) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens = Vec::default();
    let mut line_index = 0usize;
    let mut char_index = 0usize;

    let mut input = input.chars().peekable();

    while let Some(current_character) = input.next() {
        match current_character {
            '\n' | '\r' => {
                line_index += 1;
                char_index = 0;
            }
            ' ' => char_index += 1,
            '/' => {
                if let Some(next_character) = input.next() {
                    if next_character == '/' {
                        // advance iterator until the end of the line
                        while let Some(next_character) = input.peek() {
                            if matches!(next_character, '\n' | '\r') {
                                break;
                            }
                            input.next();
                        }
                        continue;
                    }
                }
                return Err(SyntaxError::new(
                    "Expected '/'. Invalid comment syntax.".to_string(),
                    line_index,
                    char_index,
                ));
            }
            _ => todo!("Implement remaining syntactical patterns"),
        }
    }

    Ok(tokens)
}

#[test]
fn tokenize_comment() {
    let input = "// Hello, this is a comment\n".to_string();

    let output = tokenize(input);
    assert!(output.is_ok());
    assert!(output.unwrap().is_empty());
}

#[test]
fn tokenize_comment_syntax_error() {
    let input = "// Hello, this is a valid comment\n/ This is an invalid comment\n".to_string();

    let output = tokenize(input);
    assert!(output.is_err());
    let output = output.unwrap_err();
    assert_eq!(output.char_index, 0);
    assert_eq!(output.line_index, 1);
}

#[derive(Clone, Debug)]
pub struct SyntaxError {
    message: String,
    line_index: usize,
    char_index: usize,
}

impl SyntaxError {
    /// Creates a new Syntax Error with the given message, line number and column number.
    pub fn new(message: String, line_index: usize, char_index: usize) -> SyntaxError {
        Self {
            message,
            line_index,
            char_index,
        }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Syntax Error: at line: {}, column: {}: {}",
            self.line_index, self.char_index, self.message
        )
    }
}

impl Error for SyntaxError {}
