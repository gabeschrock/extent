use xt_core::lex;
use xt_default::init;
use xt_interface::OperationOrder;

fn main() {
    let mut order = OperationOrder::new();
    init(&mut order);

    let tokens = lex("hello world", order).unwrap();
    println!("{tokens:#?}");
}
