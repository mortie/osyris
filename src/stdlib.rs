use super::eval::{ValRef, Scope, FuncVal, call};

use std::rc::Rc;
use std::cell::RefCell;

fn lib_print(args: &Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() {
        if idx != 0 {
            print!(" ");
        }
        print!("{}", args[idx]);
    }
    println!();
    Ok(ValRef::None)
}

fn to_num(arg: &ValRef) -> i32 {
    match arg {
        ValRef::Number(num) => *num,
        _ => 0,
    }
}

fn equals(arg1: &ValRef, arg2: &ValRef) -> bool {
    match (arg1, arg2) {
        (ValRef::None, ValRef::None) => true,
        (ValRef::Number(a), ValRef::Number(b)) => a == b,
        (ValRef::String(a), ValRef::String(b)) => a == b,
        (ValRef::Quote(a), ValRef::Quote(b)) => Rc::ptr_eq(a, b),
        (ValRef::List(a), ValRef::List(b)) => Rc::ptr_eq(a, b),
        (ValRef::Func(a), ValRef::Func(b)) => (a as *const FuncVal) == (b as *const FuncVal),
        _ => false,
    }
}

fn lib_add(args: &Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Ok(ValRef::Number(0));
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num += to_num(&args[idx]);
    }

    Ok(ValRef::Number(num))
}

fn lib_sub(args: &Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Ok(ValRef::Number(0));
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num -= to_num(&args[idx]);
    }

    Ok(ValRef::Number(num))
}

fn lib_mul(args: &Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Ok(ValRef::Number(0));
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num *= to_num(&args[idx]);
    }

    Ok(ValRef::Number(num))
}

fn lib_div(args: &Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Ok(ValRef::Number(0));
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num /= to_num(&args[idx]);
    }

    Ok(ValRef::Number(num))
}

fn lib_equals(args: &Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() <= 1 {
        return Ok(ValRef::Number(1));
    }

    for idx in 0..args.len() - 1 {
        if !equals(&args[idx], &args[idx + 1]) {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_nequals(args: &Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    match lib_equals(args, scope) {
        Ok(ValRef::Number(1)) => Ok(ValRef::Number(0)),
        Ok(ValRef::Number(0)) => Ok(ValRef::Number(1)),
        val => val,
    }
}

fn lib_lteq(args: &Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if to_num(&args[idx]) > to_num(&args[idx + 1]) {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_lt(args: &Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if to_num(&args[idx]) >= to_num(&args[idx + 1]) {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_gteq(args: &Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if to_num(&args[idx]) < to_num(&args[idx + 1]) {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_gt(args: &Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if to_num(&args[idx]) <= to_num(&args[idx + 1]) {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_set(args: &Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 2 {
        return Err("'set' requires 2 arguments".to_string());
    }

    let name = match &args[0] {
        ValRef::String(s) => s.as_ref(),
        _ => return Err("'set' requires the first argument to be a string".to_string()),
    };

    scope.borrow_mut().insert(name.clone(), args[1].clone());
    Ok(args[1].clone())
}

fn lib_if(args: &Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 2 && args.len() != 3 {
        return Err("'if' requires 2 or 3 arguments".to_string());
    }

    let cond = match args[0] {
        ValRef::Number(num) => num != 0,
        _ => false,
    };

    let expr;
    if cond {
        expr = &args[1];
    } else if args.len() == 3 {
        expr = &args[2];
    } else {
        return Ok(ValRef::None);
    }

    match expr {
        ValRef::Quote(func) => call(func, scope),
        val => Ok(val.clone()),
    }
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
    put("==", &lib_equals);
    put("!=", &lib_nequals);
    put("<=", &lib_lteq);
    put("<", &lib_lt);
    put(">=", &lib_gteq);
    put(">", &lib_gt);
    put("set", &lib_set);
    put("if", &lib_if);

    scope
}
