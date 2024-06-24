use xt_interface::*;

pub fn lex<T: ToString>(stringable: T, order: OperationOrder) -> Vec<Token> {
    let code = stringable.to_string();

    let mut lexer = Lexer {
        code,
        indices: (0, 0),
        tokens: vec![],
    };

    let mut done = false;
    while !done {
        for func in &order.lex {
            done = func(&mut lexer);
            if done { break; }
        }
    }

    lexer.tokens
}
