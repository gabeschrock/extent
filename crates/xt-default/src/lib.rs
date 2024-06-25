use std::cmp::min;

use xt_interface::{Lexer, OperationOrder, Token, MAX_SYMBOL_LEN};

pub const SKIP_WHITESPACE: &str = "https://github.com/gabeschrock/extent\nxt-default\nskip_whitespace";
pub const LEX_IDENT:       &str = "https://github.com/gabeschrock/extent\nxt-default\nlex_ident";
pub const LEX_SYMBOL:      &str = "https://github.com/gabeschrock/extent\nxt-default\nlex_symbol";
pub const PLACEHOLDER:     &str = "https://github.com/gabeschrock/extent\nxt-default\nplaceholder";

fn skip_whitespace(lexer: &mut Lexer) {
    let length = lexer.code.len();
    let chars = lexer.code[lexer.index..length].chars();
    for c in chars {
        if !c.is_whitespace() { break; }
        lexer.index += 1;
    }
}

fn lex_ident(lexer: &mut Lexer) {
    let len = lexer.code.len();
    let mut indices = (lexer.index, lexer.index + 1);
    let mut substr = &lexer.code[indices.0..indices.1];
    let first = substr
        .chars()
        .next()
        .unwrap();
    if !Token::is_ident_start_char(first) { return; }
    while Token::is_ident(substr) {
        indices.1 += 1;
        if indices.1 > len { break; }
        substr = &lexer.code[indices.0..indices.1];
    }
    indices.1 -= 1;
    substr = &lexer.code[indices.0..indices.1];
    lexer.index = indices.1;
    lexer.tokens.push(Token::Ident(String::from(substr)));
}

fn lex_symbol(lexer: &mut Lexer) {
    let end = min(lexer.index + MAX_SYMBOL_LEN, lexer.code.len());
    let mut indices = (lexer.index, end);
    while indices.0 != indices.1 {
        let substr = &lexer.code[indices.0..indices.1];
        if let Some(token) = Token::symbol(substr) {
            lexer.index = indices.1;
            lexer.tokens.push(token);
        }
        indices.1 -= 1;
    }
}

pub fn init(order: &mut OperationOrder) {
    order.lex.push((LEX_IDENT,       lex_ident      ));
    order.lex.push((LEX_SYMBOL,      lex_symbol     ));
    order.lex.push((SKIP_WHITESPACE, skip_whitespace));
}
