use super::ast;
use super::bstring::BString;

use std::any::Any;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::rc::Rc;

pub type FuncVal = dyn Fn(Vec<ValRef>, &Rc<RefCell<Scope>>) -> Result<ValRef, StackTrace>;

pub struct LambdaVal {
    pub args: Vec<BString>,
    pub body: Rc<Vec<ast::Expression>>,
}

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
    Block(Rc<Vec<ast::Expression>>),
    List(Rc<RefCell<Vec<ValRef>>>),
    Dict(Rc<RefCell<HashMap<BString, ValRef>>>),
    Func(Rc<FuncVal>),
    Lambda(Rc<LambdaVal>),
    BoundLambda(Rc<LambdaVal>, Box<ValRef>),
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
            (ValRef::Block(a), ValRef::Block(b)) => Rc::ptr_eq(a, b),
            (ValRef::Lambda(a), ValRef::Lambda(b)) => Rc::ptr_eq(a, b),
            (ValRef::List(a), ValRef::List(b)) => Rc::ptr_eq(a, b),
            (ValRef::Func(a), ValRef::Func(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }

    pub fn call_or_get(&self, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, StackTrace> {
        match self {
            ValRef::Block(..) => call(self.clone(), Vec::new(), scope),
            val => Ok(val.clone()),
        }
    }
}

impl PartialEq for ValRef {
    fn eq(&self, other: &Self) -> bool {
        ValRef::equals(self, other)
    }
}

impl Clone for ValRef {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Number(num) => Self::Number(*num),
            Self::Bool(b) => Self::Bool(*b),
            Self::String(s) => Self::String(s.clone()),
            Self::Block(q) => Self::Block(q.clone()),
            Self::List(l) => Self::List(l.clone()),
            Self::Dict(m) => Self::Dict(m.clone()),
            Self::Func(f) => Self::Func(f.clone()),
            Self::Lambda(l) => Self::Lambda(l.clone()),
            Self::BoundLambda(l, s) => Self::BoundLambda(l.clone(), s.clone()),
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
            Self::Block(q) => write!(f, "{:?}", q),
            Self::Dict(m) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, val) in m.as_ref().borrow().iter() {
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
                let vec = l.borrow();
                for idx in 0..vec.len() {
                    if idx != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", vec[idx])?;
                }
                write!(f, "]")
            }
            Self::Func(func) => write!(f, "(func {:p})", func.as_ref()),
            Self::Lambda(l) => write!(f, "(lambda {:?} {:?})", l.args, l.body),
            Self::BoundLambda(l, s) => {
                write!(f, "(bound (lambda {:?} {:?}), self={})", l.args, l.body, s)
            }
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

pub struct StackTraceEntry {
    pub location: ast::Location,
    pub name: String,
}

pub struct StackTrace {
    pub message: ValRef,
    pub trace: Vec<StackTraceEntry>,
}

impl StackTrace {
    pub fn from_str(message: &str) -> Self {
        Self {
            message: ValRef::String(Rc::new(BString::from_str(message))),
            trace: Vec::new(),
        }
    }

    pub fn from_string(message: String) -> Self {
        Self {
            message: ValRef::String(Rc::new(BString::from_string(message))),
            trace: Vec::new(),
        }
    }

    pub fn from_val(message: ValRef) -> Self {
        Self {
            message,
            trace: Vec::new(),
        }
    }

    fn push(mut self, location: ast::Location, name: String) -> Self {
        self.trace.push(StackTraceEntry { location, name });
        self
    }
}

impl fmt::Display for StackTrace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.message {
            ValRef::String(bs) => write!(f, "{}", String::from_utf8_lossy(bs.as_bytes())),
            _ => write!(f, "{}", self.message),
        }?;

        for entry in &self.trace {
            write!(
                f,
                "\n  {}: {}:{}: {}",
                entry.location.file, entry.location.line, entry.location.column, entry.name
            )?;
        }

        Ok(())
    }
}

pub struct Scope {
    pub parent: Option<Rc<RefCell<Scope>>>,
    pub map: HashMap<BString, ValRef>,
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

    pub fn lookup(&self, name: &BString) -> Option<ValRef> {
        match self.map.get(name) {
            Some(r) => Some(r.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().lookup(name),
                None => None,
            },
        }
    }

    pub fn rlookup(
        scope: &Rc<RefCell<Scope>>,
        name: &BString,
    ) -> Option<(ValRef, Rc<RefCell<Scope>>)> {
        match scope.borrow().map.get(name) {
            Some(r) => Some((r.clone(), scope.clone())),
            None => match &scope.borrow().parent {
                Some(parent) => Scope::rlookup(parent, name),
                None => None,
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

    pub fn remove(&mut self, name: &BString) {
        self.map.remove(name);
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

pub fn call(
    func: ValRef,
    mut args: Vec<ValRef>,
    scope: &Rc<RefCell<Scope>>,
) -> Result<ValRef, StackTrace> {
    match &func {
        ValRef::Func(func) => func(args, scope),
        ValRef::Block(exprs) => eval_multiple(&exprs[..], scope),
        ValRef::Lambda(l) => {
            let subscope = Rc::new(RefCell::new(Scope::new_with_parent(scope.clone())));

            {
                let mut ss = subscope.borrow_mut();

                for idx in (0..l.args.len()).rev() {
                    if idx >= args.len() {
                        break;
                    }

                    ss.insert(l.args[idx].clone(), args.pop().unwrap());
                }
            }

            eval_multiple(&l.body[..], &subscope)
        }
        ValRef::BoundLambda(l, selfval) => {
            let subscope = Rc::new(RefCell::new(Scope::new_with_parent(scope.clone())));

            {
                let mut ss = subscope.borrow_mut();

                for idx in (0..l.args.len()).rev() {
                    if idx >= args.len() {
                        break;
                    }

                    ss.insert(l.args[idx].clone(), args.pop().unwrap());
                }

                ss.insert(BString::from_str("self"), selfval.as_ref().clone());
            }

            eval_multiple(&l.body[..], &subscope)
        }
        ValRef::List(list) => {
            if args.len() != 1 {
                return Err(StackTrace::from_str("Array lookup requires 1 argument"));
            }

            let idx = match args[0] {
                ValRef::Number(idx) => idx,
                _ => {
                    return Err(StackTrace::from_str(
                        "Attempt to index array with non-number",
                    ))
                }
            };

            if idx as usize > list.borrow().len() || idx < 0.0 {
                Ok(ValRef::None)
            } else {
                Ok(list.borrow()[idx as usize].clone())
            }
        }
        ValRef::Dict(map) => {
            if args.len() != 1 {
                return Err(StackTrace::from_str(
                    "Dict lookup requires exactly 1 argument",
                ));
            }

            let key = match &args[0] {
                ValRef::String(key) => key,
                _ => return Err(StackTrace::from_str("Attempt to index map with non-string")),
            };

            match map.borrow().get(key.as_ref()) {
                Some(val) => match val {
                    ValRef::Lambda(l) => Ok(ValRef::BoundLambda(l.clone(), Box::new(func.clone()))),
                    _ => Ok(val.clone()),
                },
                None => Ok(ValRef::None),
            }
        }
        _ => Err(StackTrace::from_string(format!(
            "Attempt to call non-function {}",
            func
        ))),
    }
}

pub fn eval_call(
    exprs: &Vec<ast::Expression>,
    scope: &Rc<RefCell<Scope>>,
) -> Result<ValRef, StackTrace> {
    if exprs.len() < 1 {
        return Err(StackTrace::from_str("Call list has no elements".into()));
    }

    let mut args: Vec<ValRef> = Vec::new();
    args.reserve(exprs.len() - 1);
    for idx in 1..exprs.len() {
        args.push(eval(&exprs[idx], scope)?);
    }

    let func = eval(&exprs[0], scope)?;
    call(func, args, scope)
}

fn resolve_lazy(lazy: &ValRef, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, StackTrace> {
    match lazy {
        ValRef::Func(func) => func(Vec::new(), scope),
        ValRef::Lambda(l) => {
            let subscope = Rc::new(RefCell::new(Scope::new_with_parent(scope.clone())));
            {
                let mut ss = subscope.borrow_mut();
                ss.insert(
                    BString::from_str("args"),
                    ValRef::List(Rc::new(RefCell::new(vec![]))),
                );
            }
            eval_multiple(&l.body[..], &subscope)
        }
        ValRef::Block(exprs) => eval_multiple(exprs, &scope),
        _ => Ok(lazy.clone()),
    }
}

pub fn eval(expr: &ast::Expression, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, StackTrace> {
    let mut val = match expr {
        ast::Expression::String(s) => Ok(ValRef::String(Rc::new(s.clone()))),
        ast::Expression::Number(num) => Ok(ValRef::Number(*num)),
        ast::Expression::Lookup(name) => match scope.borrow().lookup(name) {
            Some(val) => Ok(val),
            None => Err(StackTrace::from_string(format!(
                "Variable '{}' doesn't exist",
                name
            ))),
        },
        ast::Expression::Call(exprs, loc) => {
            if exprs.len() == 0 {
                return Ok(ValRef::None);
            }

            match eval_call(exprs, scope) {
                Ok(val) => Ok(val),
                Err(trace) => Err(trace.push(loc.clone(), format!("{}", exprs[0]))),
            }
        }
        ast::Expression::Block(exprs) => Ok(ValRef::Block(exprs.clone())),
    }?;

    loop {
        match val {
            ValRef::Lazy(lazy) => val = resolve_lazy(&lazy, scope)?,
            ValRef::ProtectedLazy(lazy) => return Ok(ValRef::Lazy(lazy)),
            _ => return Ok(val),
        }
    }
}

pub fn eval_multiple(
    exprs: &[ast::Expression],
    scope: &Rc<RefCell<Scope>>,
) -> Result<ValRef, StackTrace> {
    let mut retval = ValRef::None;
    for expr in exprs {
        retval = eval(expr, scope)?;
    }

    Ok(retval)
}
