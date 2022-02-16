use super::eval::{eval_call, Scope, ValRef};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn lib_print(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() {
        if idx != 0 {
            print!(" ");
        }

        match &args[idx] {
            ValRef::String(s) => print!("{}", s.as_ref()),
            val => print!("{}", val),
        }
    }
    println!();
    Ok(ValRef::None)
}

fn to_bool(arg: &ValRef) -> bool {
    match arg {
        ValRef::Number(0) => false,
        ValRef::None => false,
        _ => true,
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

fn lib_or(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if to_bool(&args[idx]) {
            return Ok(args[idx].clone());
        }
    }

    Ok(args[args.len() - 1].clone())
}

fn lib_and(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if !to_bool(&args[idx]) {
            return Ok(args[idx].clone());
        }
    }

    Ok(args[args.len() - 1].clone())
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
        ValRef::Quote(func) => eval_call(func, scope),
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
        if !to_bool(&eval_call(cond.as_ref(), scope)?) {
            return Ok(retval);
        }

        match body {
            Some(body) => retval = eval_call(body, scope)?,
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

fn lib_bind(args: Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Err("'bind' requires at least 1 argument".to_string());
    }

    let vals = match &args[0] {
        ValRef::List(l) => l,
        _ => return Err("'bind' expects first argument to be a list".to_string()),
    };

    let mut argidx = 0;
    let mut retval = ValRef::None;
    for idx in 1..args.len() {
        let arg = &args[idx];
        match arg {
            ValRef::String(name) => {
                if argidx >= vals.len() {
                    return Err("Wrong argument count".to_string());
                }

                scope
                    .borrow_mut()
                    .insert(name.as_ref().clone(), vals[argidx].clone());
                argidx += 1;
            }
            ValRef::Quote(q) => {
                retval = eval_call(q.as_ref(), scope)?;
            }
            _ => return Err("'bind' expects strings and quotes only".to_string()),
        }
    }

    Ok(retval)
}

fn lib_list(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    Ok(ValRef::List(Rc::new(args)))
}

fn lib_map(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() % 2 != 0 {
        return Err("'map' requires an even number of arguments".to_string());
    }

    let mut map: HashMap<String, ValRef> = HashMap::new();
    let mut idx = 0;
    while idx < args.len() {
        let key = &args[idx];
        idx += 1;
        let val = &args[idx];
        idx += 1;

        let keystr = match key {
            ValRef::String(s) => s,
            _ => return Err("'map' requires keys to be strings".to_string()),
        };

        map.insert(keystr.as_ref().clone(), val.clone());
    }

    Ok(ValRef::Map(Rc::new(map)))
}

fn lib_lazy(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 1 {
        return Err("'lazy' requires 1 argument".to_string());
    }

    Ok(ValRef::ProtectedLazy(Box::new(args[0].clone())))
}

pub fn init(scope: &Rc<RefCell<Scope>>) {
    scope.borrow_mut().put_func("print", Rc::new(lib_print));
    scope.borrow_mut().put_func("+", Rc::new(lib_add));
    scope.borrow_mut().put_func("-", Rc::new(lib_sub));
    scope.borrow_mut().put_func("*", Rc::new(lib_mul));
    scope.borrow_mut().put_func("/", Rc::new(lib_div));
    scope.borrow_mut().put_func("==", Rc::new(lib_equals));
    scope.borrow_mut().put_func("!=", Rc::new(lib_nequals));
    scope.borrow_mut().put_func("<=", Rc::new(lib_lteq));
    scope.borrow_mut().put_func("<", Rc::new(lib_lt));
    scope.borrow_mut().put_func(">=", Rc::new(lib_gteq));
    scope.borrow_mut().put_func(">", Rc::new(lib_gt));
    scope.borrow_mut().put_func("||", Rc::new(lib_or));
    scope.borrow_mut().put_func("&&", Rc::new(lib_and));
    scope.borrow_mut().put_func("def", Rc::new(lib_def));
    scope.borrow_mut().put_func("set", Rc::new(lib_set));
    scope.borrow_mut().put_func("if", Rc::new(lib_if));
    scope.borrow_mut().put_func("while", Rc::new(lib_while));
    scope.borrow_mut().put_func("do", Rc::new(lib_do));
    scope.borrow_mut().put_func("bind", Rc::new(lib_bind));
    scope.borrow_mut().put_func("list", Rc::new(lib_list));
    scope.borrow_mut().put_func("map", Rc::new(lib_map));
    scope.borrow_mut().put_func("lazy", Rc::new(lib_lazy));
}
