use xt_interface::*;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error {
    TokenNotAccepted
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for Error {}

/// Lex a string into tokens
/// # Examples
/// ```
/// # use xt_core::lex;
/// # use xt_interface::{OperationOrder, Token};
/// let mut order = OperationOrder::new();
/// xt_default::init(&mut order);
/// let tokens = lex("(2 + 2) / 4", &order).unwrap();
/// match &tokens[0] {
///     Token::LParen => (),
///     other => panic!("Parse unsuccessful: {other:?}"),
/// }
/// ```
pub fn lex<T: ToString>(stringable: T, order: &OperationOrder) -> Result<Vec<Token>, Error> {
    let code = stringable.to_string();

    let mut lexer = Lexer {
        code,
        index: 0,
        tokens: vec![],
    };

    'top: loop {
        let index = lexer.index;
        for step in &order.lex {
            step.function()(&mut lexer);
            if lexer.index >= lexer.code.len() {
                break 'top;
            }
        }
        if lexer.index == index {
            return Err(Error::TokenNotAccepted);
        }
    }

    Ok(lexer.tokens)
}
