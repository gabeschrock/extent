use libloading::{Library, Symbol};
use std::env;
use xt_core::lex;
use xt_interface::OperationOrder;

macro_rules! error {
    ($code:expr, $fmt:expr) => {
        eprintln!($fmt);
        std::process::exit($code);
    };

    ($code:expr, $fmt:expr, $(arg:tt)*) => {
        eprintln!($fmt, $(arg)*);
        std::process::exit($code);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filename = match args.first() {
        Some(string) => string.as_str(),
        None => {
            error!(1, "error: filename required");
        }
    };
    
    let mut order = OperationOrder::new();
    let lib;
    unsafe {
        type InitFunction = extern "Rust" fn(&mut OperationOrder);

        lib = Library::new(filename).unwrap_or_else(|err| {
            error!(1, "failed to load library: {err}");
        });

        let init_func: Symbol<InitFunction> = lib.get(b"init").unwrap_or_else(|err| {
            error!(1, "failed to find function: {err}");
        });

        init_func(&mut order);
    }

    let tokens = lex("(hello + world * 2) - 7", order).unwrap();
    println!("{tokens:#?}");
}
