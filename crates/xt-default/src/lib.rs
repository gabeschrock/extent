use xt_interface::{Lexer, OperationOrder};

fn test(lexer: &mut Lexer) -> bool {
    println!("{lexer:#?}");
    true
}

pub fn init(order: &mut OperationOrder) {
    order.lex.push(test);
}
