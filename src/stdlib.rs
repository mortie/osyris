use super::eval::{ValRef, Scope, FuncVal, call};

use std::rc::Rc;
use std::cell::RefCell;

fn lib_print(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() {
        if idx != 0 {
            print!(" ");
        }
        print!("{}", args[idx]);
    }
    println!();
    Ok(ValRef::None)
}

fn to_bool(arg: &ValRef) -> bool {
    match arg {
        ValRef::Number(num) => *num != 0,
        _ => false,
    }
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
        (ValRef::Func(a), ValRef::Func(b)) => Rc::ptr_eq(a, b),
        _ => false,
    }
}

fn lib_add(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Ok(ValRef::Number(0));
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num += to_num(&args[idx]);
    }

    Ok(ValRef::Number(num))
}

fn lib_sub(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Ok(ValRef::Number(0));
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num -= to_num(&args[idx]);
    }

    Ok(ValRef::Number(num))
}

fn lib_mul(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Ok(ValRef::Number(0));
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num *= to_num(&args[idx]);
    }

    Ok(ValRef::Number(num))
}

fn lib_div(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Ok(ValRef::Number(0));
    }

    let mut num = to_num(&args[0]);
    for idx in 1..args.len() {
        num /= to_num(&args[idx]);
    }

    Ok(ValRef::Number(num))
}

fn lib_equals(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
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

fn lib_nequals(args: Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    match lib_equals(args, scope) {
        Ok(ValRef::Number(1)) => Ok(ValRef::Number(0)),
        Ok(ValRef::Number(0)) => Ok(ValRef::Number(1)),
        val => val,
    }
}

fn lib_lteq(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if to_num(&args[idx]) > to_num(&args[idx + 1]) {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_lt(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if to_num(&args[idx]) >= to_num(&args[idx + 1]) {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_gteq(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if to_num(&args[idx]) < to_num(&args[idx + 1]) {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_gt(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if to_num(&args[idx]) <= to_num(&args[idx + 1]) {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_def(args: Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 2 {
        return Err("'def' requires 2 arguments".to_string());
    }

    let name = match &args[0] {
        ValRef::String(s) => s.as_ref(),
        _ => return Err("'def' requires the first argument to be a string".to_string()),
    };

    scope.borrow_mut().insert(name.clone(), args[1].clone());
    Ok(ValRef::None)
}

fn lib_set(args: Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 2 {
        return Err("'set' requires 2 arguments".to_string());
    }

    let name = match &args[0] {
        ValRef::String(s) => s.as_ref(),
        _ => return Err("'set' requires the first argument to be a string".to_string()),
    };

    if scope.borrow_mut().replace(name.clone(), args[1].clone()) {
        Ok(ValRef::None)
    } else {
        Err(format!("Variable '{}' doesn't exist", name))
    }
}

fn lib_if(args: Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 2 && args.len() != 3 {
        return Err("'if' requires 2 or 3 arguments".to_string());
    }

    let expr;
    if to_bool(&args[0]) {
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

fn lib_while(args: Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 1 && args.len() != 2 {
        return Err("'while' requires 1 or 2 arguments".to_string());
    }

    let cond = match &args[0] {
        ValRef::Quote(func) => func,
        _ => return Err("'while' expects the firt argument to be a function".to_string()),
    };

    let body = if args.len() >= 1 {
        match &args[1] {
            ValRef::Quote(func) => Some(func),
            _ => return Err("'while' expects the second argument to be a function".to_string()),
        }
    } else {
        None
    };

    let mut retval: ValRef = ValRef::None;
    loop {
        if !to_bool(&call(cond.as_ref(), scope)?) {
            return Ok(retval);
        }

        match body {
            Some(body) => retval = call(body, scope)?,
            _ => (),
        };
    }
}

fn lib_do(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() > 0 {
        Ok(args[args.len() - 1].clone())
    } else {
        Ok(ValRef::None)
    }
}

fn lib_list(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    Ok(ValRef::List(Rc::new(args)))
}

fn lib_lazy(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 1 {
        return Err("'lazy' requires 1 argument".to_string());
    }

    Ok(ValRef::ProtectedLazy(Rc::new(args[0].clone())))
}

pub fn new(parent: Option<Rc<RefCell<Scope>>>) -> Scope {
    let mut scope = Scope::new(parent);

    let mut put = |name: &str, func: FuncVal| {
        scope.insert(name.to_string(), ValRef::Func(Rc::new(func)));
    };

    put("print", Box::new(lib_print));
    put("+", Box::new(lib_add));
    put("-", Box::new(lib_sub));
    put("*", Box::new(lib_mul));
    put("/", Box::new(lib_div));
    put("==", Box::new(lib_equals));
    put("!=", Box::new(lib_nequals));
    put("<=", Box::new(lib_lteq));
    put("<", Box::new(lib_lt));
    put(">=", Box::new(lib_gteq));
    put(">", Box::new(lib_gt));
    put("def", Box::new(lib_def));
    put("set", Box::new(lib_set));
    put("if", Box::new(lib_if));
    put("while", Box::new(lib_while));
    put("do", Box::new(lib_do));
    put("list", Box::new(lib_list));
    put("lazy", Box::new(lib_lazy));

    scope
}
