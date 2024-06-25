use xt_interface::{Lexer, OperationOrder, Token};

pub const LEX_IDENT: &str = "https://github.com/gabeschrock/extent\nxt-default\nlex_ident";
pub const PLACEHOLDER: &str = "https://github.com/gabeschrock/extent\nxt-default\nplaceholder";

fn placeholder(lexer: &mut Lexer) {
    let substr = &lexer.code[lexer.index..lexer.index+1];
    lexer.tokens.push(Token::Ident(String::from(substr)));
    lexer.index += 1;
}

fn lex_ident(lexer: &mut Lexer) {
    let mut indices = (lexer.index, lexer.index + 1);
    if indices.1 > lexer.code.len() - 1 { return; }
    let mut substr = &lexer.code[indices.0..indices.1];
    let first = substr
        .chars()
        .next()
        .unwrap();
    if !Token::is_ident_start_char(first) { return; }
    while Token::is_ident(substr) {
        indices.1 += 1;
        if indices.1 > lexer.code.len() { break; }
        substr = &lexer.code[indices.0..indices.1];
    }
    indices.1 -= 1;
    substr = &lexer.code[indices.0..indices.1];
    lexer.index = indices.1;
    lexer.tokens.push(Token::Ident(String::from(substr)));
}

pub fn init(order: &mut OperationOrder) {
    order.lex.push((LEX_IDENT, lex_ident));
    order.lex.push((PLACEHOLDER, placeholder));
}
