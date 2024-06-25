use xt_interface::*;
use std::fmt::Debug;

pub enum ErrorKind {
    TokenizeLoopError
}

pub struct Error {
    kind: ErrorKind,
}

impl Error {
    fn new(kind: ErrorKind) -> Error {
        Error {
            kind,
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorKind::*;
        write!(f, "{}", match &self.kind {
            TokenizeLoopError => "An infinite loop was detected in the tokenizer"
        })
    }
}

pub fn lex<T: ToString>(stringable: T, order: OperationOrder) -> Result<Vec<Token>, Error> {
    let code = stringable.to_string();

    let mut lexer = Lexer {
        code,
        index: 0,
        tokens: vec![],
    };

    'top: loop {
        let index = lexer.index;
        for (_, func) in &order.lex {
            func(&mut lexer);
            if lexer.index >= lexer.code.len() {
                break 'top;
            }
        }
        if lexer.index == index {
            return Err(Error::new(ErrorKind::TokenizeLoopError));
        }
    }

    Ok(lexer.tokens)
}
