use token::{
    GeneralPurposeRegister, InstructionType, RegisterType, SegmentRegister, SpecialPurposeRegister,
    TokenType,
};

use crate::lexer::token::Token;
use std::{error::Error, fmt::Display, vec::Vec};

mod test;
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
            'a'..='z' | 'A'..='Z' => {
                let start_index = char_index;

                let mut buffer = String::from(current_character);
                char_index += 1;

                while let Some(next_character) = input.peek() {
                    if !next_character.is_alphabetic() {
                        break;
                    }

                    buffer.push(*next_character);
                    input.next();
                    char_index += 1;
                }

                // parse instruction & registers
                if let Some(token_type) = parse_token(buffer.as_str()) {
                    tokens.push(Token::new(
                        token_type,
                        line_index,
                        start_index,
                        char_index - start_index,
                    ));
                }

                // parse labels
                if let Some(next_character) = input.peek() {
                    if *next_character == ':' {
                        buffer.push(input.next().unwrap());
                        char_index += 1;

                        tokens.push(Token::new(
                            TokenType::Label(buffer),
                            line_index,
                            start_index,
                            char_index - start_index,
                        ));
                    }
                }
            }
            // parse comma
            ',' => {
                tokens.push(Token::new(TokenType::Comma, line_index, char_index, 1));
                char_index += 1;
            }
            // parse memory location
            '[' => {
                let start_index = char_index;

                let mut buffer = String::default();
                char_index += 1;

                while let Some(next_character) = input.peek() {
                    if !next_character.is_ascii_alphanumeric() {
                        break;
                    }

                    buffer.push(*next_character);

                    input.next();
                    char_index += 1;
                }

                if let Some(next_character) = input.next() {
                    if next_character == ']' {
                        char_index += 1;

                        // hex
                        let (buffer, radix) = if buffer.starts_with("0x") {
                            (&buffer[2..], 16)
                        } else if buffer.ends_with("h") {
                            (&buffer[0..buffer.len() - 1], 16)
                        }
                        // dec
                        else {
                            (buffer.as_str(), 10)
                        };

                        let value = u16::from_str_radix(buffer, radix).map_err(|_| {
                            SyntaxError::new(
                                "Expected 16-bit memory address. Invalid memory location syntax."
                                    .to_string(),
                                line_index,
                                start_index,
                            )
                        })?;

                        tokens.push(Token::new(
                            TokenType::MemoryLocation(value),
                            line_index,
                            start_index,
                            char_index - start_index,
                        ));

                        continue;
                    }
                }

                return Err(SyntaxError::new(
                    "Expected ']'. Invalid memory location syntax.".to_string(),
                    line_index,
                    char_index,
                ));
            }
            _ => Err(SyntaxError::new(
                format!("Unexpected character: '{}'", current_character),
                line_index,
                char_index,
            ))?,
        }
    }

    Ok(tokens)
}

fn parse_token(buffer: &str) -> Option<TokenType> {
    match buffer.to_lowercase().as_str() {
        // Instruction types
        "aaa" => Some(TokenType::Instruction(InstructionType::Aaa)),
        "aad" => Some(TokenType::Instruction(InstructionType::Aad)),
        "aam" => Some(TokenType::Instruction(InstructionType::Aam)),
        "aas" => Some(TokenType::Instruction(InstructionType::Aas)),
        "adc" => Some(TokenType::Instruction(InstructionType::Adc)),
        "add" => Some(TokenType::Instruction(InstructionType::Add)),
        "and" => Some(TokenType::Instruction(InstructionType::And)),
        "call" => Some(TokenType::Instruction(InstructionType::Call)),
        "cbw" => Some(TokenType::Instruction(InstructionType::Cbw)),
        "clc" => Some(TokenType::Instruction(InstructionType::Clc)),
        "cld" => Some(TokenType::Instruction(InstructionType::Cld)),
        "cli" => Some(TokenType::Instruction(InstructionType::Cli)),
        "cmc" => Some(TokenType::Instruction(InstructionType::Cmc)),
        "cmp" => Some(TokenType::Instruction(InstructionType::Cmp)),
        "cmpsb" => Some(TokenType::Instruction(InstructionType::Cmpsb)),
        "cmpsw" => Some(TokenType::Instruction(InstructionType::Cmpsw)),
        "cwd" => Some(TokenType::Instruction(InstructionType::Cwd)),
        "daa" => Some(TokenType::Instruction(InstructionType::Daa)),
        "das" => Some(TokenType::Instruction(InstructionType::Das)),
        "dec" => Some(TokenType::Instruction(InstructionType::Dec)),
        "div" => Some(TokenType::Instruction(InstructionType::Div)),
        "esc" => Some(TokenType::Instruction(InstructionType::Esc)),
        "hlt" => Some(TokenType::Instruction(InstructionType::Hlt)),
        "idiv" => Some(TokenType::Instruction(InstructionType::Idiv)),
        "imul" => Some(TokenType::Instruction(InstructionType::Imul)),
        "in" => Some(TokenType::Instruction(InstructionType::In)),
        "inc" => Some(TokenType::Instruction(InstructionType::Inc)),
        "int" => Some(TokenType::Instruction(InstructionType::Int)),
        "into" => Some(TokenType::Instruction(InstructionType::Into)),
        "iret" => Some(TokenType::Instruction(InstructionType::Iret)),
        "jcc" => Some(TokenType::Instruction(InstructionType::Jcc)),
        "jcxz" => Some(TokenType::Instruction(InstructionType::Jcxz)),
        "jmp" => Some(TokenType::Instruction(InstructionType::Jmp)),
        "lahf" => Some(TokenType::Instruction(InstructionType::Lahf)),
        "lds" => Some(TokenType::Instruction(InstructionType::Lds)),
        "lea" => Some(TokenType::Instruction(InstructionType::Lea)),
        "les" => Some(TokenType::Instruction(InstructionType::Les)),
        "lock" => Some(TokenType::Instruction(InstructionType::Lock)),
        "lodsb" => Some(TokenType::Instruction(InstructionType::Lodsb)),
        "lodsw" => Some(TokenType::Instruction(InstructionType::Lodsw)),
        "loop" => Some(TokenType::Instruction(InstructionType::Loop)),
        "mov" => Some(TokenType::Instruction(InstructionType::Mov)),
        "movsb" => Some(TokenType::Instruction(InstructionType::Movsb)),
        "movsw" => Some(TokenType::Instruction(InstructionType::Movsw)),
        "mul" => Some(TokenType::Instruction(InstructionType::Mul)),
        "neg" => Some(TokenType::Instruction(InstructionType::Neg)),
        "nop" => Some(TokenType::Instruction(InstructionType::Nop)),
        "not" => Some(TokenType::Instruction(InstructionType::Not)),
        "or" => Some(TokenType::Instruction(InstructionType::Or)),
        "out" => Some(TokenType::Instruction(InstructionType::Out)),
        "pop" => Some(TokenType::Instruction(InstructionType::Pop)),
        "popf" => Some(TokenType::Instruction(InstructionType::Popf)),
        "push" => Some(TokenType::Instruction(InstructionType::Push)),
        "pushf" => Some(TokenType::Instruction(InstructionType::Pushf)),
        "rcl" => Some(TokenType::Instruction(InstructionType::Rcl)),
        "rcr" => Some(TokenType::Instruction(InstructionType::Rcr)),
        "rep" => Some(TokenType::Instruction(InstructionType::Rep)),
        "ret" => Some(TokenType::Instruction(InstructionType::Ret)),
        "rol" => Some(TokenType::Instruction(InstructionType::Rol)),
        "ror" => Some(TokenType::Instruction(InstructionType::Ror)),
        "sahf" => Some(TokenType::Instruction(InstructionType::Sahf)),
        "sal" => Some(TokenType::Instruction(InstructionType::Sal)),
        "sar" => Some(TokenType::Instruction(InstructionType::Sar)),
        "sbb" => Some(TokenType::Instruction(InstructionType::Sbb)),
        "scasb" => Some(TokenType::Instruction(InstructionType::Scasb)),
        "scasw" => Some(TokenType::Instruction(InstructionType::Scasw)),
        "shl" => Some(TokenType::Instruction(InstructionType::Shl)),
        "shr" => Some(TokenType::Instruction(InstructionType::Shr)),
        "stc" => Some(TokenType::Instruction(InstructionType::Stc)),
        "std" => Some(TokenType::Instruction(InstructionType::Std)),
        "sti" => Some(TokenType::Instruction(InstructionType::Sti)),
        "stosb" => Some(TokenType::Instruction(InstructionType::Stosb)),
        "stosw" => Some(TokenType::Instruction(InstructionType::Stosw)),
        "sub" => Some(TokenType::Instruction(InstructionType::Sub)),
        "test" => Some(TokenType::Instruction(InstructionType::Test)),
        "wait" => Some(TokenType::Instruction(InstructionType::Wait)),
        "xchg" => Some(TokenType::Instruction(InstructionType::Xchg)),
        "xlat" => Some(TokenType::Instruction(InstructionType::Xlat)),
        "xor" => Some(TokenType::Instruction(InstructionType::Xor)),

        // General Purpose Registers
        "al" => Some(TokenType::Register(RegisterType::GeneralPurpose(
            GeneralPurposeRegister::Al,
        ))),
        "ah" => Some(TokenType::Register(RegisterType::GeneralPurpose(
            GeneralPurposeRegister::Ah,
        ))),
        "ax" => Some(TokenType::Register(RegisterType::GeneralPurpose(
            GeneralPurposeRegister::Ax,
        ))),
        "bl" => Some(TokenType::Register(RegisterType::GeneralPurpose(
            GeneralPurposeRegister::Bl,
        ))),
        "bh" => Some(TokenType::Register(RegisterType::GeneralPurpose(
            GeneralPurposeRegister::Bh,
        ))),
        "bx" => Some(TokenType::Register(RegisterType::GeneralPurpose(
            GeneralPurposeRegister::Bx,
        ))),
        "cl" => Some(TokenType::Register(RegisterType::GeneralPurpose(
            GeneralPurposeRegister::Cl,
        ))),
        "ch" => Some(TokenType::Register(RegisterType::GeneralPurpose(
            GeneralPurposeRegister::Ch,
        ))),
        "cx" => Some(TokenType::Register(RegisterType::GeneralPurpose(
            GeneralPurposeRegister::Cx,
        ))),
        "dl" => Some(TokenType::Register(RegisterType::GeneralPurpose(
            GeneralPurposeRegister::Dl,
        ))),
        "dh" => Some(TokenType::Register(RegisterType::GeneralPurpose(
            GeneralPurposeRegister::Dh,
        ))),
        "dx" => Some(TokenType::Register(RegisterType::GeneralPurpose(
            GeneralPurposeRegister::Dx,
        ))),

        // Segment Registers
        "cs" => Some(TokenType::Register(RegisterType::Segment(
            SegmentRegister::Cs,
        ))),
        "ds" => Some(TokenType::Register(RegisterType::Segment(
            SegmentRegister::Ds,
        ))),
        "ss" => Some(TokenType::Register(RegisterType::Segment(
            SegmentRegister::Ss,
        ))),
        "es" => Some(TokenType::Register(RegisterType::Segment(
            SegmentRegister::Es,
        ))),

        // Special Purpose Registers
        "sp" => Some(TokenType::Register(RegisterType::SpecialPurpose(
            SpecialPurposeRegister::Sp,
        ))),
        "bp" => Some(TokenType::Register(RegisterType::SpecialPurpose(
            SpecialPurposeRegister::Bp,
        ))),
        "si" => Some(TokenType::Register(RegisterType::SpecialPurpose(
            SpecialPurposeRegister::Si,
        ))),
        "di" => Some(TokenType::Register(RegisterType::SpecialPurpose(
            SpecialPurposeRegister::Di,
        ))),
        "ip" => Some(TokenType::Register(RegisterType::SpecialPurpose(
            SpecialPurposeRegister::Ip,
        ))),

        _ => None,
    }
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
