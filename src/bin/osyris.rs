use osyris::{eval, importlib, iolib, parse, stdlib};
use std::cell::RefCell;
use std::env;
use std::fs;
use std::rc::Rc;
use std::process;

fn usage(argv0: &String) {
    println!("Usage: {} [options] <path>", argv0);
    println!("Options:");
    println!("  --help, -h:  Show this help text");
    println!("  --print-ast: Print the syntax tree instead of executing");
}

fn main() {
    let mut args = env::args();
    let argv0 = args.next().unwrap();

    let mut path: Option<String> = None;
    let mut print_ast = false;
    let mut dashes = false;
    while let Some(arg) = args.next() {
        if !dashes && (arg == "--help" || arg == "-h") {
            usage(&argv0);
            return;
        } else if !dashes && (arg == "--print-ast") {
            print_ast = true;
        } else if !dashes && arg == "--" {
            dashes = true;
        } else if path.is_none() {
            path = Some(arg);
        } else {
            eprintln!("Unexpected arguemnt: {}", arg);
            process::exit(1);
        }
    }

    let path = match path {
        Some(path) => path,
        None => {
            usage(&argv0);
            process::exit(1);
        }
    };

    let string = match fs::read_to_string(&path) {
        Ok(string) => string,
        Err(err) => {
            eprintln!("{}: {}", path, err);
            return;
        }
    };

    let mut reader = parse::Reader::new(&string.as_bytes());

    let scope = Rc::new(RefCell::new(eval::Scope::new()));
    stdlib::init(&scope);
    iolib::init(&scope);
    importlib::init_with_path(&scope, &path);

    loop {
        let expr = match parse::parse(&mut reader) {
            Ok(expr) => match expr {
                Some(expr) => expr,
                None => break,
            },
            Err(err) => {
                eprintln!("Parse error: {}:{}: {}", err.line, err.col, err.msg);
                process::exit(1);
            }
        };

        if print_ast {
            println!("{}", expr);
        } else {
            match eval::eval(&expr, &scope) {
                Err(err) => {
                    eprintln!("Error: {}", err);
                    process::exit(1);
                }
                _ => (),
            }
        }
    }
}
