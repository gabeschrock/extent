use std::cmp::min;

use xt_interface::*;

pub const URL: &str = "https://github.com/gabeschrock/extent";
pub const LOCATION: &str = "xt-default";
pub const SKIP_WHITESPACE: &str = "https://github.com/gabeschrock/extent\nxt-default\nskip_whitespace";
pub const LEX_IDENT:       &str = "https://github.com/gabeschrock/extent\nxt-default\nlex_ident";
pub const LEX_SYMBOL:      &str = "https://github.com/gabeschrock/extent\nxt-default\nlex_symbol";
pub const PLACEHOLDER:     &str = "https://github.com/gabeschrock/extent\nxt-default\nplaceholder";

pub fn skip_whitespace(lexer: &mut Lexer) {
    let length = lexer.code.len();
    let chars = lexer.code[lexer.index..length].chars();
    for c in chars {
        if !c.is_whitespace() { break; }
        lexer.index += 1;
    }
}

pub fn lex_ident(lexer: &mut Lexer) {
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

pub fn lex_num_literal(lexer: &mut Lexer) {
    // TODO: more advanced number literal parsing
    let mut indices = (lexer.index, lexer.index + 1);
    let mut substr = &lexer.code[indices.0..indices.1];
    let first = substr
        .chars()
        .next()
        .unwrap();
    if !first.is_digit(10) { return; }
    while substr
        .chars()
        .last()
        .unwrap()
        .is_numeric()
    {
        indices.1 += 1;
        substr = &lexer.code[indices.0..indices.1];
    }
    indices.1 -= 1;
    substr = &lexer.code[indices.0..indices.1];
    lexer.index = indices.1;
    lexer.tokens.push(Token::NumLiteral(substr.parse::<FSize>().unwrap()))
}

pub fn lex_symbol(lexer: &mut Lexer) {
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

#[no_mangle]
pub extern "Rust" fn init(order: &mut OperationOrder) {
    let owner = StepOwner::new(URL, LOCATION, "");
    let skip_whitespace_step: LexerStep = Step::new(owner.new_name("skip_whitespace"), skip_whitespace);
    let lex_ident_step:       LexerStep = Step::new(owner.new_name("lex_ident"),       lex_ident );
    let lex_num_literal_step: LexerStep = Step::new(owner.new_name("lex_num_literal"), lex_num_literal);
    let lex_symbol_step:      LexerStep = Step::new(owner.new_name("lex_symbol"),      lex_symbol);
   
    order.lex.push(skip_whitespace_step);
    order.lex.push(lex_ident_step);
    order.lex.push(skip_whitespace_step);
    order.lex.push(lex_num_literal_step);
    order.lex.push(skip_whitespace_step);
    order.lex.push(lex_symbol_step);
}
