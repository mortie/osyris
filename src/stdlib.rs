use super::eval::{ValRef, Scope};

use std::rc::Rc;

fn lib_print(args: &Vec<ValRef>) -> ValRef {
    for idx in 0..args.len() {
        if idx != 0 {
            print!(" ");
        }
        print!("{}", args[idx]);
    }
    println!();
    ValRef::None
}

fn to_num(arg: &ValRef) -> i32 {
    match arg {
        ValRef::Number(num) => *num,
        _ => 0,
    }
}

fn lib_add(args: &Vec<ValRef>) -> ValRef {
    if args.len() < 1 {
        return ValRef::Number(0);
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num += to_num(&args[idx]);
    }

    ValRef::Number(num)
}

fn lib_sub(args: &Vec<ValRef>) -> ValRef {
    if args.len() < 1 {
        return ValRef::Number(0);
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num -= to_num(&args[idx]);
    }

    ValRef::Number(num)
}

fn lib_mul(args: &Vec<ValRef>) -> ValRef {
    if args.len() < 1 {
        return ValRef::Number(0);
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num *= to_num(&args[idx]);
    }

    ValRef::Number(num)
}

fn lib_div(args: &Vec<ValRef>) -> ValRef {
    if args.len() < 1 {
        return ValRef::Number(0);
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num /= to_num(&args[idx]);
    }

    ValRef::Number(num)
}

pub fn new(parent: Option<Rc<Scope>>) -> Scope {
    let mut scope = Scope::new(parent);

    let mut put = |name: &str, func: &'static dyn Fn(&Vec<ValRef>) -> ValRef | {
        scope.insert(name.to_string(), ValRef::Func(func));
    };

    put("print", &lib_print);
    put("+", &lib_add);
    put("-", &lib_sub);
    put("*", &lib_mul);
    put("/", &lib_div);

    scope
}
