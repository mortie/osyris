use super::ast;

use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;
use std::cell::RefCell;

pub type FuncVal = Box<dyn Fn(Vec<ValRef>, &Rc<RefCell<Scope>>) -> Result<ValRef, String>>;

pub enum ValRef {
    None,
    Number(i32),
    String(Rc<String>),
    Quote(Rc<Vec<ast::Expression>>),
    List(Rc<Vec<ValRef>>),
    Func(Rc<FuncVal>),
    Lazy(Rc<ValRef>),
    ProtectedLazy(Rc<ValRef>),
}

impl Clone for ValRef {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Number(num) => Self::Number(*num),
            Self::String(s) => Self::String(s.clone()),
            Self::Quote(q) => Self::Quote(q.clone()),
            Self::List(l) => Self::List(l.clone()),
            Self::Func(f) => Self::Func(f.clone()),
            Self::Lazy(val) => Self::Lazy(val.clone()),
            Self::ProtectedLazy(val) => Self::ProtectedLazy(val.clone()),
        }
    }
}

impl fmt::Display for ValRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Number(num) => write!(f, "{}", num),
            Self::String(s) => write!(f, "{}", s),
            Self::Quote(q) => write!(f, "{:?}", q),
            Self::List(l) => {
                write!(f, "[")?;
                let vec = l.as_ref();
                for idx in 0..vec.len() {
                    if idx != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", vec[idx])?;
                }
                write!(f, "]")
            }
            Self::Func(_) => write!(f, "(func)"),
            Self::Lazy(val) => write!(f, "(lazy {})", val),
            Self::ProtectedLazy(val) => write!(f, "(protected-lazy {})", val),
        }
    }
}

pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    map: HashMap<String, ValRef>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Scope>>>) -> Self {
        Self {
            parent,
            map: HashMap::new(),
        }
    }

    fn lookup(&self, name: &String) -> Result<ValRef, String> {
        match self.map.get(name) {
            Some(r) => Ok(r.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().lookup(name),
                None => Err(format!("Variable '{}' doesn't exist", name)),
            }
        }
    }

    pub fn insert(&mut self, name: String, val: ValRef) {
        self.map.insert(name, val);
    }
}

pub fn call(exprs: &Vec<ast::Expression>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if exprs.len() < 1 {
        return Err("Call list has no elements".to_string());
    }

    let mut args: Vec<ValRef> = Vec::new();
    args.reserve(exprs.len() - 1);
    for idx in 1..exprs.len() {
        args.push(eval(&exprs[idx], scope)?);
    }

    let func = eval(&exprs[0], scope)?;
    match func {
        ValRef::Func(func) => func(args, scope),
        ValRef::Quote(exprs) => {
            let s = Rc::new(RefCell::new(Scope::new(Some(scope.clone()))));
            s.borrow_mut().insert("$".to_string(), ValRef::List(Rc::new(args)));

            let mut retval = ValRef::None;
            for expr in exprs.as_ref() {
                retval = eval(expr, &s)?;
            }

            Ok(retval)
        }
        ValRef::List(list) => {
            if args.len() != 1 {
                return Err(format!("Array lookup requires exactly 1 argument, got {}", args.len()));
            }

            let idx = match args[0] {
                ValRef::Number(idx) => idx,
                _ => return Err("Attempt to index with non-number".to_string()),
            };

            if idx as usize > list.len() || idx < 0 {
                Ok(ValRef::None)
            } else {
                Ok(list.as_ref()[idx as usize].clone())
            }
        }
        _ => Err(format!("Attempt to call non-function {}", func))
    }
}

fn resolve_lazy(lazy: &ValRef, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    match lazy {
        ValRef::Func(func) => {
            let args: Vec<ValRef> = Vec::new();
            func(args, scope)
        }
        ValRef::Quote(exprs) => {
            let s = Rc::new(RefCell::new(Scope::new(Some(scope.clone()))));

            let mut retval = ValRef::None;
            for expr in exprs.as_ref() {
                retval = eval(expr, &s)?;
            }

            Ok(retval)
        }
        _ => Ok(lazy.clone()),
    }
}

pub fn eval(expr: &ast::Expression, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    let mut val = match expr {
        ast::Expression::String(s) => Ok(ValRef::String(Rc::new(s.clone()))),
        ast::Expression::Number(num) => Ok(ValRef::Number(*num)),
        ast::Expression::Lookup(name) => scope.borrow().lookup(name),
        ast::Expression::Call(exprs) => call(exprs, scope),
        ast::Expression::Quote(exprs) => Ok(ValRef::Quote(exprs.clone())),
    }?;

    loop {
        match val {
            ValRef::Lazy(lazy) => val = resolve_lazy(&lazy, scope)?,
            ValRef::ProtectedLazy(lazy) => return Ok(ValRef::Lazy(lazy)),
            _ => return Ok(val),
        }
    }
}
