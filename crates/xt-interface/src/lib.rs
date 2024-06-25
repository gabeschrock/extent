use std::fmt::Debug;

pub const MAX_SYMBOL_LEN: usize = 1;

#[cfg(target_pointer_width = "64")]
pub type FSize = f64;

#[cfg(target_pointer_width = "32")]
pub type FSize = f32;

#[derive(Debug)]
pub enum Token {
    Ident(String),
    StringLiteral(String),
    NumLiteral(FSize),
    OtherToken(Box<dyn OtherToken>),

    // Punctuators
    LParen,
    RParen,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug)]
pub struct Lexer {
    pub code: String,
    pub index: usize,
    pub tokens: Vec<Token>,
}

pub struct OperationOrder<'a> {
    pub lex: Vec<(&'a str, fn(&mut Lexer))>
}

pub trait OtherToken: Debug {}

impl Token {
    pub fn is_ident_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == '$'
    }

    pub fn is_ident_start_char(c: char) -> bool {
        c.is_alphabetic() || c == '_' || c == '$'
    }

    pub fn is_ident(string: &str) -> bool {
        if string.len() == 0 { return true; }
        let mut chars = string.chars();
        if !Token::is_ident_start_char(chars.next().unwrap()) {
            return false;
        };
        for c in chars {
            if !Token::is_ident_char(c) { return false; }
        }
        true
    }

    pub fn symbol(string: &str) -> Option<Token> {
        use Token::*;
        return Some(match string {
            "(" => LParen,
            ")" => RParen,
            "+" => Plus,
            "-" => Minus,
            "*" => Star,
            "/" => Slash,
            _   => return None,
        })
    }
}

impl<'a> OperationOrder<'a> {
    pub fn new() -> OperationOrder<'a> {
        OperationOrder {
            lex: vec![],
        }
    }
}
