use libloading::{Library, Symbol};
use std::env;
use xt_core::lex;
use xt_interface::OperationOrder;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filename = args[0].as_str();
    
    let mut order = OperationOrder::new();
    let lib;
    unsafe {
        type InitFunction = extern "Rust" fn(&mut OperationOrder);

        lib = Library::new(filename).unwrap_or_else(|err| {
            panic!("Failed to load library: {:?}", err);
        });

        // Get a symbol for the function `init`
        let init_func: Symbol<InitFunction> = lib.get(b"init").unwrap_or_else(|err| {
            panic!("Failed to load function: {:?}", err);
        });


        init_func(&mut order);
    }

    let tokens = lex("(hello + world)", order).unwrap();
    println!("{tokens:#?}");
}
