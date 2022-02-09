use super::eval::{ValRef, Scope, FuncVal};

use std::rc::Rc;
use std::cell::RefCell;

fn lib_print(args: &Vec<ValRef>, _: &RefCell<Scope>) -> ValRef {
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

fn lib_add(args: &Vec<ValRef>, _: &RefCell<Scope>) -> ValRef {
    if args.len() < 1 {
        return ValRef::Number(0);
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num += to_num(&args[idx]);
    }

    ValRef::Number(num)
}

fn lib_sub(args: &Vec<ValRef>, _: &RefCell<Scope>) -> ValRef {
    if args.len() < 1 {
        return ValRef::Number(0);
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num -= to_num(&args[idx]);
    }

    ValRef::Number(num)
}

fn lib_mul(args: &Vec<ValRef>, _: &RefCell<Scope>) -> ValRef {
    if args.len() < 1 {
        return ValRef::Number(0);
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num *= to_num(&args[idx]);
    }

    ValRef::Number(num)
}

fn lib_div(args: &Vec<ValRef>, _: &RefCell<Scope>) -> ValRef {
    if args.len() < 1 {
        return ValRef::Number(0);
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num /= to_num(&args[idx]);
    }

    ValRef::Number(num)
}

fn lib_set(args: &Vec<ValRef>, scope: &RefCell<Scope>) -> ValRef {
    if args.len() != 2 {
        return ValRef::None;
    }

    let name = match &args[0] {
        ValRef::String(s) => s.as_ref(),
        _ => return ValRef::None,
    };

    scope.borrow_mut().insert(name.clone(), args[1].clone());
    args[1].clone()
}

pub fn new(parent: Option<Rc<RefCell<Scope>>>) -> Scope {
    let mut scope = Scope::new(parent);

    let mut put = |name: &str, func: FuncVal| {
        scope.insert(name.to_string(), ValRef::Func(func));
    };

    put("print", &lib_print);
    put("+", &lib_add);
    put("-", &lib_sub);
    put("*", &lib_mul);
    put("/", &lib_div);
    put("set", &lib_set);

    scope
}
