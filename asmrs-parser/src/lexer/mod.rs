use crate::lexer::token::Token;
use std::vec::Vec;

pub mod token;

// todo: return Result<Vec<Token>, SyntaxError> instead
pub fn tokenize(input: String) -> Vec<Token> {
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
                todo!("Syntax Error Handling: Comment invalid")
            }
            _ => todo!("Implement remaining syntactical patterns"),
        }
    }

    tokens
}

#[test]
fn tokenize_comment() {
    let input = "// Hello, this is a comment\n".to_string();

    let output = tokenize(input);

    assert!(output.is_empty());
}
