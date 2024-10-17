/// Token representation of assembly code.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    /// Type of token
    r#type: TokenType,
    /// Line of occurrence
    line_index: usize,
    /// Character index of occurrence
    char_index: usize,
    /// Length of tokenin chars
    char_len: usize,
}

/// Types of tokens
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    Instruction(InstructionType), // mov, add, xor, ...
    Register(RegisterType),       // ax, bx, si, di, ...
    Constant(u16),                // 1234h, ...
    MemoryLocation(u16),          // [0xdeadbeef], [0xcafebabe], ...
    Label(String),                // hello:, MSG:, ...
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionType {
    /// ASCII adjust AL after addition
    Aaa,
    /// ASCII adjust AX before division
    Aad,
    /// ASCII adjust AX after multiplication
    Aam,
    /// ASCII adjust AL after subtraction
    Aas,
    /// Add with carry
    Adc,
    /// Add
    Add,
    /// Logical And
    And,
    /// Call procedure
    Call,
    /// Convert byte to word
    Cbw,
    /// Clear carry flag
    Clc,
    /// Clear direction flag
    Cld,
    /// Clear interrupt flag
    Cli,
    /// Complement carry flag
    Cmc,
    /// Compare operands
    Cmp,
    /// Compare bytes in memory
    Cmpsb,
    /// Compare words in memory
    Cmpsw,
    /// Convert word to doubleword
    Cwd,
    /// Decimal adjust AL after addition
    Daa,
    /// Decimal adjust AL after subtraction
    Das,
    /// Decrement by 1
    Dec,
    /// Unsigned divide
    Div,
    /// Used with floating point unit
    Esc,
    /// Enter hlt state
    Hlt,
    /// Signed divide
    Idiv,
    /// Signed multiply in One-Operand form
    Imul,
    /// Input from port
    In,
    /// Increment by 1
    Inc,
    /// Call to interrupt
    Int,
    /// Call to interrupt if overflow
    Into,
    /// Return from interrupt
    Iret,
    /// Jump if condition
    Jcc,
    /// Jump if CX is zero
    Jcxz,
    /// Jump
    Jmp,
    /// Load FLAGS into AH register
    Lahf,
    /// Load DS:r with far pointer
    Lds,
    /// Load effective address
    Lea,
    /// Load ES:r with far pointer
    Les,
    /// Assert BUS LOCK# signal
    Lock,
    /// Load string byte
    Lodsb,
    /// Load string word
    Lodsw,
    /// Loop control
    Loop,
    /// Move data
    Mov,
    /// Move byte from string to string
    Movsb,
    /// Move word from string to string
    Movsw,
    /// Unsigned multiply
    Mul,
    /// Two's complement negation
    Neg,
    /// No operation
    Nop,
    /// Negate operand, logical NOT
    Not,
    /// Logical OR
    Or,
    /// Output to port
    Out,
    /// Pop data from stack
    Pop,
    /// Pop FLAGS register from stack
    Popf,
    /// Push data onto stack
    Push,
    /// Push FLAGS onto stack
    Pushf,
    /// Rotate left (with carry)
    Rcl,
    /// Rotate right (with carry)
    Rcr,
    /// Repeat instructions (REPxx)
    Rep,
    /// Return from procedure
    Ret,
    /// Rotate left
    Rol,
    /// Rotate right
    Ror,
    /// Store AH into FLAGS
    Sahf,
    /// Shift arithmetically left
    Sal,
    /// Shift arithmetically right
    Sar,
    /// Subtract with borrow
    Sbb,
    /// Compare byte string
    Scasb,
    /// Compare word string
    Scasw,
    /// Shift left
    Shl,
    /// Shift right
    Shr,
    /// Set carry flag
    Stc,
    /// Set direction flag
    Std,
    /// Set interrupt flag
    Sti,
    /// Store byte in string
    Stosb,
    /// Store word in string
    Stosw,
    /// Subtract
    Sub,
    /// Logical compare (AND)
    Test,
    /// Wait until not busy
    Wait,
    /// Exchange data
    Xchg,
    /// Table look-up translation
    Xlat,
    /// Exclusive OR
    Xor,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegisterType {
    GeneralPurpose(GeneralPurposeRegister),
    Segment(SegmentRegister),
    SpecialPurpose(SpecialPurposeRegister),
}

/// Versitile
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GeneralPurposeRegister {
    /// Lower 8 bits of Accumulator Register
    Al,
    /// Higher 8 bits of Accumulator Register
    Ah,
    /// Accumulator Register
    Ax,

    /// Lower 8 bits of Base Register
    Bl,
    /// Higher 8 bits of Base Register
    Bh,
    /// Base Register
    Bx,

    /// Lower 8 bits of Counter Register
    Cl,
    /// Higher 8 bits of Counter Register
    Ch,
    /// Counter Register
    Cx,

    /// Lower 8 bits of Data Register
    Dl,
    /// Higher 8 bits of Data Register
    Dh,
    /// Data Register
    Dx,
}

/// Responsible for managing memory access
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SegmentRegister {
    /// Code Segment
    Cs,
    /// Data Segment
    Ds,
    /// Stack Segment
    Ss,
    /// Extra Segment
    Es,
}

/// Index and Pointer Registers
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecialPurposeRegister {
    /// Stack Pointer (top of stack)
    Sp,
    /// Base Pointer (bottom of stack)
    Bp,
    /// Source Index (memory pointer, used to store offset address of a source)
    Si,
    /// Destination Index (memory pointer, used to store offset address of a destination)
    Di,
    /// Instruaction Pointer (address of next instruction to be exectured)
    Ip,
}
