use std::fmt::Debug;

pub type LexerStep<'a> = Step<'a, fn(&mut Lexer)>;

#[cfg(target_pointer_width = "64")]
pub type FSize = f64;

#[cfg(target_pointer_width = "32")]
pub type FSize = f32;

pub const MAX_SYMBOL_LEN: usize = 1;

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

#[derive(Clone, Copy, Debug)]
pub struct StepOwner<'a> {
    url: &'a str,
    location: &'a str,
    name: &'a str,
}

impl<'a> StepOwner<'a> {
    pub fn new(url: &'a str, location: &'a str, name: &'a str) -> Self {
        StepOwner {
            url,
            location,
            name,
        }
    }

    pub fn url(&self) -> &'a str {
        self.url
    }

    pub fn location(&self) -> &'a str {
        self.location
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn new_name(&self, name: &'a str) -> Self {
        Self {
            name,
            ..*self
        }
    }
}

impl<'a> Default for StepOwner<'a> {
    fn default() -> Self {
        Self::new("about:blank", "", "")
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Step<'a, T> 
where
    T: Clone,
{
    owner: StepOwner<'a>,
    function: T,
}

impl<'a, T> Step<'a, T>
where 
    T: Clone,
{
    pub fn new(owner: StepOwner<'a>, function: T) -> Self {
        Self {
            owner,
            function,
        }
    }

    pub fn owner(&self) -> StepOwner {
        self.owner
    }

    pub fn function(&self) -> T {
        self.function.clone()
    }
}

#[derive(Debug)]
pub struct Lexer {
    pub code: String,
    pub index: usize,
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct OperationOrder<'a> {
    pub lex: Vec<LexerStep<'a>>
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
        if string.is_empty() { return true; }
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
        Some(match string {
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
    pub fn new() -> Self {
        OperationOrder {
            lex: vec![],
        }
    }
}

impl<'a> Default for OperationOrder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
