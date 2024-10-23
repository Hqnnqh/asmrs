#[test]
fn tokenize_comment() {
    use crate::lexer::tokenize;
    let input = "// Hello, this is a comment\n".to_string();

    let output = tokenize(input);
    assert!(output.is_ok());
    assert!(output.unwrap().is_empty());
}

#[test]
fn tokenize_comment_syntax_error() {
    use crate::lexer::tokenize;

    let input = "// Hello, this is a valid comment\n/ This is an invalid comment\n".to_string();

    let output = tokenize(input);
    assert!(output.is_err());
    let output = output.unwrap_err();
    assert_eq!(output.char_index, 0);
    assert_eq!(output.line_index, 1);
}

#[test]
fn tokenize_instruction() {
    use crate::lexer::{
        tokenize, InstructionType, RegisterType, SpecialPurposeRegister, Token, TokenType,
    };

    let input = "mov sp, bp".to_string();

    let output = tokenize(input);
    assert!(output.is_ok());
    let output = output.unwrap();

    // mov
    assert_eq!(
        output[0],
        Token::new(TokenType::Instruction(InstructionType::Mov), 0, 0, 3)
    );

    // sp
    assert_eq!(
        output[1],
        Token::new(
            TokenType::Register(RegisterType::SpecialPurpose(SpecialPurposeRegister::Sp)),
            0,
            4,
            2
        )
    );

    // ,
    assert_eq!(output[2], Token::new(TokenType::Comma, 0, 6, 1));

    // bp
    assert_eq!(
        output[3],
        Token::new(
            TokenType::Register(RegisterType::SpecialPurpose(SpecialPurposeRegister::Bp)),
            0,
            8,
            2
        )
    );
}

#[test]
fn tokenize_label() {
    use crate::lexer::{tokenize, Token, TokenType};

    let input = "mylabel:".to_string();
    let output = tokenize(input);
    assert!(output.is_ok());
    let output = output.unwrap();
    assert_eq!(
        output[0],
        Token::new(TokenType::Label("mylabel:".to_string()), 0, 0, 8)
    );
}

#[test]
fn tokenize_memory_location() {
    use crate::lexer::{tokenize, Token, TokenType};

    let input = "[beefh]".to_string();
    let output = tokenize(input);
    assert!(output.is_ok());
    let output = output.unwrap();
    assert_eq!(
        output[0],
        Token::new(TokenType::MemoryLocation(0xbeef), 0, 0, 7)
    )
}

#[test]
fn tokenize_all() {
    use crate::lexer::{
        tokenize, GeneralPurposeRegister, InstructionType, RegisterType, SpecialPurposeRegister,
        Token, TokenType,
    };

    let input = r#"// this is a fun comment
        mylabel:
            mov ax, bx
            inc
            lea [0xbeef], di
        "#
    .to_string();

    let output = tokenize(input);
    assert!(output.is_ok());
    let output = output.unwrap();

    // comment is not added to tokens

    // label
    assert_eq!(
        output[0],
        Token::new(TokenType::Label("mylabel:".to_string()), 1, 8, 8)
    );

    // mov
    assert_eq!(
        output[1],
        Token::new(TokenType::Instruction(InstructionType::Mov), 2, 12, 3)
    );

    // ax
    assert_eq!(
        output[2],
        Token::new(
            TokenType::Register(RegisterType::GeneralPurpose(GeneralPurposeRegister::Ax)),
            2,
            16,
            2
        )
    );

    // comma
    assert_eq!(output[3], Token::new(TokenType::Comma, 2, 18, 1));

    // bx
    assert_eq!(
        output[4],
        Token::new(
            TokenType::Register(RegisterType::GeneralPurpose(GeneralPurposeRegister::Bx)),
            2,
            20,
            2
        )
    );

    // inc
    assert_eq!(
        output[5],
        Token::new(TokenType::Instruction(InstructionType::Inc), 3, 12, 3)
    );

    // lea
    assert_eq!(
        output[6],
        Token::new(TokenType::Instruction(InstructionType::Lea), 4, 12, 3)
    );

    // [0xbeef]
    assert_eq!(
        output[7],
        Token::new(TokenType::MemoryLocation(0xbeef), 4, 16, 8)
    );

    // comma
    assert_eq!(output[8], Token::new(TokenType::Comma, 4, 24, 1));

    // di
    assert_eq!(
        output[9],
        Token::new(
            TokenType::Register(RegisterType::SpecialPurpose(SpecialPurposeRegister::Di)),
            4,
            26,
            2
        )
    );
}
