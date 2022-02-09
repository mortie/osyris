mod ast;
mod parse;
mod eval;
mod stdlib;

use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    args.next().unwrap();

    let path = match args.next() {
        Some(path) => path,
        None => {
            println!("Need argument");
            return;
        }
    };

    let string = match fs::read_to_string(&path) {
        Ok(string) => string,
        Err(err) => {
            println!("{}: {}", path, err);
            return;
        }
    };

    let mut reader = parse::Reader::new(&string.as_bytes());

    let scope = stdlib::new(None);
    loop {
        let expr = match parse::parse(&mut reader) {
            Ok(expr) => match expr {
                Some(expr) => expr,
                None => break,
            }
            Err(err) => {
                println!("Parse error: {}:{}: {}", err.line, err.col, err.msg);
                return;
            }
        };

        match eval::eval(&expr, &scope) {
            Err(err) => {
                println!("Error: {}", err);
                return;
            }
            _ => ()
        }
    }
}
