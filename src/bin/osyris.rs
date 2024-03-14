use osyris::{bstring::BString, dotlib, eval, importlib, iolib, parse, stdlib};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::process;

fn usage(argv0: &OsStr) {
    println!("Usage: {:?} [options] <path>", argv0);
    println!("Options:");
    println!("  --help, -h:  Show this help text");
    println!("  --print-ast: Print the syntax tree instead of executing");
}

fn main() {
    let mut args = env::args_os();
    let argv0 = args.next().unwrap();

    let mut path: Option<BString> = None;
    let mut print_ast = false;
    let mut dashes = false;
    for arg in args {
        if !dashes && (arg == "--help" || arg == "-h") {
            usage(&argv0);
            return;
        }
        
        if !dashes && (arg == "--print-ast") {
            print_ast = true;
        } else if !dashes && arg == "--" {
            dashes = true;
        } else if path.is_none() {
            path = Some(BString::from_os_str(&arg));
        } else {
            eprintln!("Unexpected arguemnt: {:?}", arg);
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

    let string = match fs::read(path.to_path()) {
        Ok(string) => string,
        Err(err) => {
            eprintln!("{}: {}", path, err);
            return;
        }
    };

    let mut reader = parse::Reader::new(&string, path.clone());

    let mut rootscope = eval::Scope::new();
    rootscope = stdlib::init(rootscope);
    rootscope = iolib::init(rootscope);
    rootscope = importlib::init_with_path(rootscope, path);
    rootscope = dotlib::init(rootscope);

    let mut scope = rootscope.subscope();

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
            match eval::eval(&expr, scope) {
                Ok((_, s)) => scope = s,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    process::exit(1);
                }
            }
        }
    }
}
