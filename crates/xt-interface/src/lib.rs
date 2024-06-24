#[derive(Debug)]
pub enum Token {
    Ident(String),
    StringLiteral(String),
    NumLiteral(f64),

    // Punctuators
    LParen,
    RParen,

    // Operators
    Plus,
    Minus,
    Slash,
    Star,
}

#[derive(Debug)]
pub struct Lexer {
    pub code: String,
    pub indices: (usize, usize),
    pub tokens: Vec<Token>,
}

pub struct OperationOrder {
    pub lex: Vec<fn(&mut Lexer) -> bool>
}

impl OperationOrder {
    pub fn new() -> OperationOrder {
        OperationOrder {
            lex: vec![],
        }
    }
}
