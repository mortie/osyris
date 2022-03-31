use super::ast;
use super::bstring::BString;

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::rc::Rc;

pub type FuncVal = dyn Fn(Vec<ValRef>, &Rc<RefCell<Scope>>) -> Result<ValRef, String>;

pub trait PortVal {
    fn read(&mut self) -> Result<ValRef, String> {
        return Err("This port doesn't support reading".to_string());
    }

    fn read_chunk(&mut self, _: usize) -> Result<ValRef, String> {
        return Err("This port doesn't support reading chunks".to_string());
    }

    fn write(&mut self, _: &ValRef) -> Result<(), String> {
        return Err("This port doesn't support writing".to_string());
    }

    fn seek(&mut self, _: io::SeekFrom) -> Result<(), String> {
        return Err("This port doesn't support seeking".to_string());
    }
}

pub enum ValRef {
    None,
    Number(f64),
    Bool(bool),
    String(Rc<BString>),
    Quote(Rc<Vec<ast::Expression>>),
    List(Rc<Vec<ValRef>>),
    Map(Rc<HashMap<BString, ValRef>>),
    Func(Rc<FuncVal>),
    Lazy(Rc<ValRef>),
    ProtectedLazy(Rc<ValRef>),
    Native(Rc<dyn Any>),
    Port(Rc<RefCell<dyn PortVal>>),
}

impl ValRef {
    pub fn to_bool(&self) -> bool {
        match self {
            ValRef::Bool(false) => false,
            _ => true,
        }
    }

    pub fn to_num(&self) -> f64 {
        match self {
            ValRef::Number(num) => *num,
            ValRef::Bool(b) => {
                if *b {
                    1f64
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    pub fn equals(a: &Self, b: &Self) -> bool {
        match (a, b) {
            (ValRef::None, ValRef::None) => true,
            (ValRef::Number(a), ValRef::Number(b)) => a == b,
            (ValRef::String(a), ValRef::String(b)) => a == b,
            (ValRef::Quote(a), ValRef::Quote(b)) => Rc::ptr_eq(a, b),
            (ValRef::List(a), ValRef::List(b)) => Rc::ptr_eq(a, b),
            (ValRef::Func(a), ValRef::Func(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }

    pub fn call_or_get(&self, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
        match self {
            ValRef::Quote(..) => call(self.clone(), vec![], scope),
            val => Ok(val.clone()),
        }
    }
}

impl Clone for ValRef {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Number(num) => Self::Number(*num),
            Self::Bool(b) => Self::Bool(*b),
            Self::String(s) => Self::String(s.clone()),
            Self::Quote(q) => Self::Quote(q.clone()),
            Self::List(l) => Self::List(l.clone()),
            Self::Map(m) => Self::Map(m.clone()),
            Self::Func(f) => Self::Func(f.clone()),
            Self::Lazy(val) => Self::Lazy(val.clone()),
            Self::ProtectedLazy(val) => Self::ProtectedLazy(val.clone()),
            Self::Native(n) => Self::Native(n.clone()),
            Self::Port(p) => Self::Port(p.clone()),
        }
    }
}

impl fmt::Display for ValRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Number(num) => write!(f, "{}", num),
            Self::Bool(b) => write!(f, "{}", b),
            Self::String(s) => write!(f, "{:?}", s),
            Self::Quote(q) => write!(f, "{:?}", q),
            Self::Map(m) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, val) in m.as_ref() {
                    if !first {
                        write!(f, ", ")?;
                    }

                    write!(f, "{:?}: {}", key, val)?;
                    first = false;
                }
                write!(f, "}}")
            }
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
            Self::Func(func) => write!(f, "(func {:p})", func.as_ref()),
            Self::Lazy(val) => write!(f, "(lazy {})", val),
            Self::ProtectedLazy(val) => write!(f, "(protected-lazy {})", val),
            Self::Native(n) => write!(f, "(native {:p})", n.as_ref()),
            Self::Port(p) => write!(f, "(port {:p})", p.as_ref()),
        }
    }
}

impl fmt::Debug for ValRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    map: HashMap<BString, ValRef>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            parent: None,
            map: HashMap::new(),
        }
    }

    pub fn new_with_parent(parent: Rc<RefCell<Scope>>) -> Self {
        Self {
            parent: Some(parent),
            map: HashMap::new(),
        }
    }

    pub fn lookup(&self, name: &BString) -> Result<ValRef, String> {
        match self.map.get(name) {
            Some(r) => Ok(r.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().lookup(name),
                None => Err(format!("Variable '{}' doesn't exist", name)),
            },
        }
    }

    pub fn insert(&mut self, name: BString, val: ValRef) {
        self.map.insert(name, val);
    }

    pub fn replace(&mut self, name: BString, val: ValRef) -> bool {
        if self.map.contains_key(&name) {
            self.map.insert(name, val);
            true
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().replace(name, val)
        } else {
            false
        }
    }

    pub fn put(&mut self, name: &str, val: ValRef) {
        self.map.insert(BString::from_str(name), val);
    }

    pub fn put_lazy(&mut self, name: &str, func: Rc<FuncVal>) {
        self.map.insert(
            BString::from_str(name),
            ValRef::Lazy(Rc::new(ValRef::Func(func))),
        );
    }

    pub fn put_func(&mut self, name: &str, func: Rc<FuncVal>) {
        self.map.insert(BString::from_str(name), ValRef::Func(func));
    }
}

pub fn call(func: ValRef, args: Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    match func {
        ValRef::Func(func) => func(args, scope),
        ValRef::Quote(exprs) => {
            let s = Rc::new(RefCell::new(Scope::new_with_parent(scope.clone())));
            s.borrow_mut()
                .insert(BString::from_str("args"), ValRef::List(Rc::new(args)));

            let mut retval = ValRef::None;
            for expr in exprs.as_ref() {
                retval = eval(expr, &s)?;
            }

            Ok(retval)
        }
        ValRef::List(list) => {
            if args.len() != 1 {
                return Err("Array lookup requires 1 argument".to_string());
            }

            let idx = match args[0] {
                ValRef::Number(idx) => idx,
                _ => return Err("Attempt to index array with non-number".to_string()),
            };

            if idx as usize > list.len() || idx < 0.0 {
                Ok(ValRef::None)
            } else {
                Ok(list.as_ref()[idx as usize].clone())
            }
        }
        ValRef::Map(map) => {
            if args.len() != 1 {
                return Err("Map lookup requires exactly 1 argument".to_string());
            }

            let key = match &args[0] {
                ValRef::String(key) => key,
                _ => return Err("Attempt to index map with non-string".to_string()),
            };

            match map.as_ref().get(key.as_ref()) {
                Some(val) => Ok(val.clone()),
                None => Ok(ValRef::None),
            }
        }
        _ => Err(format!("Attempt to call non-function {}", func)),
    }
}

pub fn eval_call(
    exprs: &Vec<ast::Expression>,
    scope: &Rc<RefCell<Scope>>,
) -> Result<ValRef, String> {
    if exprs.len() < 1 {
        return Err("Call list has no elements".to_string());
    }

    let mut args: Vec<ValRef> = Vec::new();
    args.reserve(exprs.len() - 1);
    for idx in 1..exprs.len() {
        args.push(eval(&exprs[idx], scope)?);
    }

    let func = eval(&exprs[0], scope)?;
    call(func, args, scope)
}

fn resolve_lazy(lazy: &ValRef, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    match lazy {
        ValRef::Func(func) => {
            let args: Vec<ValRef> = Vec::new();
            func(args, scope)
        }
        ValRef::Quote(exprs) => {
            let s = Rc::new(RefCell::new(Scope::new_with_parent(scope.clone())));

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
        ast::Expression::Call(exprs) => eval_call(exprs, scope),
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
