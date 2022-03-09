use super::eval::{eval_call, Scope, ValRef};

use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
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

fn lib_add(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Ok(ValRef::Number(0));
    }

    let mut num = args[0].to_num();
    for idx in 1..args.len() {
        num += &args[idx].to_num();
    }

    Ok(ValRef::Number(num))
}

fn lib_sub(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Ok(ValRef::Number(0));
    }

    let mut num = args[0].to_num();
    for idx in 1..args.len() {
        num -= args[idx].to_num();
    }

    Ok(ValRef::Number(num))
}

fn lib_mul(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Ok(ValRef::Number(0));
    }

    let mut num = args[0].to_num();
    for idx in 1..args.len() {
        num *= args[idx].to_num();
    }

    Ok(ValRef::Number(num))
}

fn lib_div(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() < 1 {
        return Ok(ValRef::Number(0));
    }

    let mut num = args[0].to_num();
    for idx in 1..args.len() {
        num /= args[idx].to_num();
    }

    Ok(ValRef::Number(num))
}

fn lib_equals(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() <= 1 {
        return Ok(ValRef::Number(1));
    }

    for idx in 0..args.len() - 1 {
        if !ValRef::equals(&args[idx], &args[idx + 1]) {
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
        if args[idx].to_num() > args[idx + 1].to_num() {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_lt(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if args[idx].to_num() >= args[idx + 1].to_num() {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_gteq(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if args[idx].to_num() < args[idx + 1].to_num() {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_gt(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if args[idx].to_num() <= args[idx + 1].to_num() {
            return Ok(ValRef::Number(0));
        }
    }

    Ok(ValRef::Number(1))
}

fn lib_or(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if args[idx].to_bool() {
            return Ok(args[idx].clone());
        }
    }

    Ok(args[args.len() - 1].clone())
}

fn lib_and(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    for idx in 0..args.len() - 1 {
        if !args[idx].to_bool() {
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
    if args[0].to_bool() {
        expr = &args[1];
    } else if args.len() == 3 {
        expr = &args[2];
    } else {
        return Ok(ValRef::None);
    }

    expr.call_or_get(scope)
}

fn lib_match(args: Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    let mut idx = 0;
    while idx < args.len() {
        // If we hit the last argument, that's the default case
        if idx == args.len() - 1 {
            return args[idx].call_or_get(scope);
        }

        let cond = &args[idx].call_or_get(scope)?;
        idx += 1;
        let body = &args[idx];
        idx += 1;

        if cond.to_bool() {
            return body.call_or_get(scope);
        }
    }

    Ok(ValRef::None)
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
        if !eval_call(cond.as_ref(), scope)?.to_bool() {
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
    for idx in 1..args.len() - 1 {
        let arg = &args[idx];
        match arg {
            ValRef::String(name) => {
                if argidx >= vals.len() {
                    return Err("'bind': Wrong argument count".to_string());
                }

                scope
                    .borrow_mut()
                    .insert(name.as_ref().clone(), vals[argidx].clone());
                argidx += 1;
            }
            _ => return Err("'bind' expects strings only".to_string()),
        }
    }

    match &args[args.len() - 1] {
        ValRef::Quote(q) => eval_call(q.as_ref(), scope),
        _ => return Err("'bind' expects its last argument to be a quote".to_string()),
    }
}

fn lib_with(args: Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    let mut idx = 0;
    while idx < args.len() - 1 {
        let name = match &args[idx] {
            ValRef::String(s) => s,
            _ => return Err("'with' requires names to be string".to_string()),
        };

        idx += 1;
        let val = &args[idx];
        idx += 1;

        scope
            .borrow_mut()
            .insert(name.as_ref().clone(), val.clone());
    }

    match &args[args.len() - 1] {
        ValRef::Quote(q) => eval_call(q.as_ref(), scope),
        _ => return Err("'bind' expects its last argument to be a quote".to_string()),
    }
}

fn lib_list(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    Ok(ValRef::List(Rc::new(args)))
}

fn lib_dict(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
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

    Ok(ValRef::ProtectedLazy(Rc::new(args[0].clone())))
}

fn lib_read(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 1 && args.len() != 2 {
        return Err("'read' requires 1 or 2 arguments".to_string());
    }

    let port = match &args[0] {
        ValRef::Port(port) => port,
        _ => return Err("'read' requires the first argument to be a port".to_string()),
    };

    if args.len() == 1 {
        port.borrow_mut().read()
    } else {
        let size = match args[1] {
            ValRef::Number(num) => num,
            _ => return Err("'read' requires the second argument to be a number".to_string()),
        };
        port.borrow_mut().read_chunk(size as usize)
    }
}

fn lib_write(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 2 {
        return Err("'write' requires 2 arguments".to_string());
    }

    let port = match &args[0] {
        ValRef::Port(port) => port,
        _ => return Err("'write' requires the first argument to be a port".to_string()),
    };

    port.borrow_mut().write(&args[1])?;
    Ok(ValRef::None)
}

fn lib_seek(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 2 && args.len() != 3 {
        return Err("'seek' requires 2 or 3 arguments".to_string());
    }

    let port = match &args[0] {
        ValRef::Port(port) => port,
        _ => return Err("'seek' requires the first argument to be a port".to_string()),
    };

    let num = match &args[1] {
        ValRef::Number(num) => *num,
        _ => return Err("'seek' requires the second argument to be a number".to_string()),
    };

    let pos = if args.len() == 2 {
        io::SeekFrom::Start(num as u64)
    } else {
        let name = match &args[2] {
            ValRef::String(s) => s.as_ref(),
            _ => return Err("'seek' requires the third argument to be a string".to_string()),
        };

        match name.as_str() {
            "set" => io::SeekFrom::Start(num as u64),
            "end" => io::SeekFrom::End(num as i64),
            "current" => io::SeekFrom::Current(num as i64),
            _ => {
                return Err(
                    "'seek' requires the seek offset to be 'set', 'end' or 'current'".to_string(),
                )
            }
        }
    };

    port.borrow_mut().seek(pos)?;
    Ok(ValRef::None)
}

pub fn init(scope: &Rc<RefCell<Scope>>) {
    let mut s = scope.borrow_mut();
    s.put("none", ValRef::None);
    s.put_func("print", Rc::new(lib_print));
    s.put_func("+", Rc::new(lib_add));
    s.put_func("-", Rc::new(lib_sub));
    s.put_func("*", Rc::new(lib_mul));
    s.put_func("/", Rc::new(lib_div));
    s.put_func("==", Rc::new(lib_equals));
    s.put_func("!=", Rc::new(lib_nequals));
    s.put_func("<=", Rc::new(lib_lteq));
    s.put_func("<", Rc::new(lib_lt));
    s.put_func(">=", Rc::new(lib_gteq));
    s.put_func(">", Rc::new(lib_gt));
    s.put_func("||", Rc::new(lib_or));
    s.put_func("&&", Rc::new(lib_and));
    s.put_func("def", Rc::new(lib_def));
    s.put_func("set", Rc::new(lib_set));
    s.put_func("if", Rc::new(lib_if));
    s.put_func("match", Rc::new(lib_match));
    s.put_func("while", Rc::new(lib_while));
    s.put_func("do", Rc::new(lib_do));
    s.put_func("bind", Rc::new(lib_bind));
    s.put_func("with", Rc::new(lib_with));
    s.put_func("list", Rc::new(lib_list));
    s.put_func("dict", Rc::new(lib_dict));
    s.put_func("lazy", Rc::new(lib_lazy));
    s.put_func("read", Rc::new(lib_read));
    s.put_func("write", Rc::new(lib_write));
    s.put_func("seek", Rc::new(lib_seek));
}
