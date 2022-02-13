use glisp::{parse, eval, stdlib};
use std::env;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

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

    let scope = Rc::new(RefCell::new(stdlib::new(None)));
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