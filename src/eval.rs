use super::ast;
use super::bstring::BString;

use std::any::Any;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::rc::Rc;
use std::vec;

pub trait FuncArgs {
    fn next_val(&mut self) -> Result<ValRef, StackTrace>;
    fn has_next(&self) -> bool;
    fn done(&self) -> Result<(), StackTrace>;
}

impl FuncArgs for vec::Drain<'_, ValRef> {
    fn next_val(&mut self) -> Result<ValRef, StackTrace> {
        match self.next() {
            Some(val) => Ok(val),
            None => Err(StackTrace::from_str("Not enough parameters")),
        }
    }

    fn has_next(&self) -> bool {
        self.len() > 0
    }

    fn done(&self) -> Result<(), StackTrace> {
        if self.has_next() {
            Err(StackTrace::from_str("Too many arguments"))
        } else {
            Ok(())
        }
    }
}

pub type FuncResult = Result<(ValRef, Scope), StackTrace>;
pub type DictVal = HashMap<BString, ValRef>;
pub type FuncVal = dyn Fn(Vec<ValRef>, Scope) -> FuncResult;

pub struct LambdaVal {
    pub args: Vec<BString>,
    pub body: Rc<Vec<ast::Expression>>,
}

pub trait PortVal {
    fn read(&mut self) -> Result<ValRef, String> {
        Err("This port doesn't support reading".to_string())
    }

    fn read_chunk(&mut self, _: usize) -> Result<ValRef, String> {
        Err("This port doesn't support reading chunks".to_string())
    }

    fn write(&mut self, _: &ValRef) -> Result<(), String> {
        Err("This port doesn't support writing".to_string())
    }

    fn seek(&mut self, _: io::SeekFrom) -> Result<(), String> {
        Err("This port doesn't support seeking".to_string())
    }
}

pub enum ValRef {
    None,
    Number(f64),
    Bool(bool),
    String(Rc<BString>),
    Block(Rc<Vec<ast::Expression>>),
    List(Rc<RefCell<Vec<ValRef>>>),
    Dict(Rc<RefCell<DictVal>>),
    Func(Rc<FuncVal>),
    Lambda(Rc<LambdaVal>),
    Binding(Rc<HashMap<BString, ValRef>>, Rc<ValRef>),
    Lazy(Rc<ValRef>),
    ProtectedLazy(Rc<ValRef>),
    Native(Rc<dyn Any>),
    Port(Rc<RefCell<dyn PortVal>>),
}

impl ValRef {
    pub fn to_bool(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            ValRef::Bool(false) => false,
            ValRef::None => false,
            _ => true,
        }
    }

    pub fn to_num(&self) -> f64 {
        match self {
            ValRef::Number(num) => *num,
            ValRef::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    pub fn to_bstring(&self) -> BString {
        // TODO: Implement display in terms of to_bstring
        // instead of the other way around maybe?
        if let ValRef::String(s) = self {
            s.as_ref().clone()
        } else {
            BString::from_string(format!("{}", self))
        }
    }

    pub fn equals(a: &Self, b: &Self) -> bool {
        #[allow(clippy::vtable_address_comparisons)]
        match (a, b) {
            (ValRef::None, ValRef::None) => true,
            (ValRef::Number(a), ValRef::Number(b)) => a == b,
            (ValRef::Bool(a), ValRef::Bool(b)) => a == b,
            (ValRef::String(a), ValRef::String(b)) => a == b,
            (ValRef::Block(a), ValRef::Block(b)) => Rc::ptr_eq(a, b),
            (ValRef::List(a), ValRef::List(b)) => {
                let (a, b) = (a.borrow(), b.borrow());
                if a.len() != b.len() {
                    return false;
                }

                for idx in 0..a.len() {
                    if !ValRef::equals(&a[idx], &b[idx]) {
                        return false;
                    }
                }

                true
            }
            (ValRef::Dict(a), ValRef::Dict(b)) => {
                let (a, b) = (a.borrow(), b.borrow());
                if a.len() != b.len() {
                    return false;
                }

                for key in a.keys() {
                    let aval = &a[key];
                    match b.get(key) {
                        Some(bval) => {
                            if !ValRef::equals(aval, bval) {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }

                true
            }
            (ValRef::Func(a), ValRef::Func(b)) => Rc::ptr_eq(a, b),
            (ValRef::Lambda(a), ValRef::Lambda(b)) => Rc::ptr_eq(a, b),
            (ValRef::Binding(a1, a2), ValRef::Binding(b1, b2)) => {
                Rc::ptr_eq(a1, b1) && Rc::ptr_eq(a2, b2)
            }
            (ValRef::Lazy(a), ValRef::Lazy(b)) => Rc::ptr_eq(a, b),
            (ValRef::ProtectedLazy(a), ValRef::ProtectedLazy(b)) => Rc::ptr_eq(a, b),
            (ValRef::Native(a), ValRef::Native(b)) => Rc::ptr_eq(a, b),
            (ValRef::Port(a), ValRef::Port(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }

    pub fn get_number(self) -> Result<f64, StackTrace> {
        match self {
            ValRef::Number(n) => Ok(n),
            _ => Err(StackTrace::from_str("Expected number")),
        }
    }

    pub fn get_string(self) -> Result<Rc<BString>, StackTrace> {
        match self {
            ValRef::String(s) => Ok(s),
            _ => Err(StackTrace::from_str("Expected string")),
        }
    }

    pub fn get_block(self) -> Result<Rc<Vec<ast::Expression>>, StackTrace> {
        match self {
            ValRef::Block(b) => Ok(b),
            _ => Err(StackTrace::from_str("Expected block")),
        }
    }

    pub fn get_list(self) -> Result<Rc<RefCell<Vec<ValRef>>>, StackTrace> {
        match self {
            ValRef::List(l) => Ok(l),
            _ => Err(StackTrace::from_str("Expected list")),
        }
    }

    pub fn get_dict(self) -> Result<Rc<RefCell<DictVal>>, StackTrace> {
        match self {
            ValRef::Dict(d) => Ok(d),
            _ => Err(StackTrace::from_str("Expected dict")),
        }
    }

    pub fn get_port(self) -> Result<Rc<RefCell<dyn PortVal>>, StackTrace> {
        match self {
            ValRef::Port(p) => Ok(p),
            _ => Err(StackTrace::from_str("Expected port")),
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
            Self::Binding(l, s) => Self::Binding(l.clone(), s.clone()),
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
            Self::Binding(b, func) => {
                write!(f, "(binding {:?} {:?})", *b, *func)
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
    #[allow(clippy::should_implement_trait)]
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

pub struct ScopeImpl {
    pub parent: Option<Rc<ScopeImpl>>,
    pub map: Option<Rc<RefCell<HashMap<BString, ValRef>>>>,
}

impl ScopeImpl {
    pub fn lookup(&self, name: &BString) -> Option<ValRef> {
        if let Some(map) = &self.map {
            if let Some(val) = map.borrow().get(name) {
                return Some(val.clone());
            }
        }

        if let Some(parent) = &self.parent {
            return parent.lookup(name);
        }

        None
    }
}

pub struct Scope {
    pub m: Rc<ScopeImpl>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            m: Rc::new(ScopeImpl {
                parent: None,
                map: None,
            }),
        }
    }

    pub fn subscope(&self) -> Self {
        Self {
            m: Rc::new(ScopeImpl {
                parent: Some(self.m.clone()),
                map: None,
            })
        }
    }

    pub fn lookup(&self, name: &BString) -> Option<ValRef> {
        self.m.lookup(name)
    }

    pub fn lookup_shallow(&self, name: &BString) -> Option<ValRef> {
        if let Some(map) = &self.m.map {
            return map.borrow().get(name).map(|x| x.clone());
        }

        None
    }

    // Erase a key from the scope if doing so wouldn't require creating a new scope.
    // If erasing would require creating a new scope, it's left 
    pub fn maybe_inplace_erase(&mut self, name: &BString) {
        if let Some(map) = &self.m.map {
            if Rc::strong_count(map) == 1 && Rc::strong_count(&self.m) == 1 {
                map.borrow_mut().remove(name);
            }
        }
    }

    pub fn insert(self, name: BString, val: ValRef) -> Self {
        if let Some(map) = &self.m.map {
            if Rc::strong_count(map) == 1 && Rc::strong_count(&self.m) == 1 {
                map.borrow_mut().insert(name, val);
                self
            } else {
                let mut map = map.borrow().clone();
                map.insert(name, val);
                Self {
                    m: Rc::new(ScopeImpl {
                        parent: self.m.parent.clone(),
                        map: Some(Rc::new(RefCell::new(map))),
                    })
                }
            }
        } else {
            let mut map = HashMap::new();
            map.insert(name, val);
            Self {
                m: Rc::new(ScopeImpl {
                    parent: self.m.parent.clone(),
                    map: Some(Rc::new(RefCell::new(map))),
                }),
            }
        }
    }

    pub fn has_shallow(&self, name: &BString) -> bool {
        if let Some(map) = &self.m.map {
            map.borrow().contains_key(name)
        } else {
            false
        }
    }

    pub fn put(self, name: &str, val: ValRef) -> Self {
        self.insert(BString::from_str(name), val)
    }

    pub fn put_lazy(self, name: &str, func: Rc<FuncVal>) -> Self {
        self.insert(BString::from_str(name), ValRef::Lazy(Rc::new(ValRef::Func(func))))
    }

    pub fn put_func(self, name: &str, func: Rc<FuncVal>) -> Self {
        self.insert(BString::from_str(name), ValRef::Func(func))
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

pub fn call(func: &ValRef, mut args: Vec<ValRef>, scope: Scope) -> FuncResult {
    match &func {
        ValRef::Func(func) => func(args, scope),
        ValRef::Block(exprs) => eval_multiple(&exprs[..], scope),
        ValRef::Lambda(l) => {
            let mut args = args.drain(0..);

            let mut subscope = scope.subscope();
            for name in &l.args {
                let val = match args.next() {
                    Some(val) => val,
                    None => break,
                };

                subscope = subscope.insert(name.clone(), val);
            }

            let (retval, _) = eval_multiple(&l.body[..], subscope)?;
            Ok((retval, scope))
        }
        ValRef::Binding(b, func) => {
            let mut subscope = scope.subscope();
            for (key, val) in b.as_ref() {
                subscope = subscope.insert(key.clone(), val.clone());
            }

            call(func.as_ref(), args, subscope)
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

            if idx as usize >= list.borrow().len() || idx < 0.0 {
                Ok((ValRef::None, scope))
            } else {
                Ok((list.borrow()[idx as usize].clone(), scope))
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
                Some(val) => Ok((val.clone(), scope)),
                None => Ok((ValRef::None, scope)),
            }
        }
        _ => Err(StackTrace::from_string(format!(
            "Attempt to call non-function {}",
            func
        ))),
    }
}

pub fn eval_call(exprs: &[ast::Expression], mut scope: Scope) -> FuncResult {
    if exprs.is_empty() {
        return Err(StackTrace::from_str("Call list has no elements"));
    }

    let mut args: Vec<ValRef> = Vec::new();
    args.reserve(exprs.len() - 1);
    for item in exprs.iter().skip(1) {
        let arg;
        (arg, scope) = eval(item, scope)?;
        args.push(arg);
    }

    let (func, scope) = eval(&exprs[0], scope)?;
    call(&func, args, scope)
}

fn resolve_lazy(lazy: &ValRef, scope: Scope) -> FuncResult {
    match lazy {
        ValRef::Func(func) => func(Vec::new(), scope),
        ValRef::Lambda(l) => {
            eval_multiple(&l.body[..], scope.subscope())
        }
        ValRef::Block(exprs) => eval_multiple(exprs, scope),
        _ => Ok((lazy.clone(), scope)),
    }
}

pub fn eval(expr: &ast::Expression, scope: Scope) -> FuncResult {
    let (mut val, mut scope) = match expr {
        ast::Expression::String(s) => Ok((ValRef::String(Rc::new(s.clone())), scope)),
        ast::Expression::Number(num) => Ok((ValRef::Number(*num), scope)),
        ast::Expression::Lookup(name) => match scope.lookup(name) {
            Some(val) => Ok((val, scope)),
            None => Err(StackTrace::from_string(format!(
                "Variable '{}' doesn't exist",
                name
            ))),
        },
        ast::Expression::Call(exprs, loc) => match eval_call(exprs, scope) {
            Ok(res) => Ok(res),
            Err(trace) => Err(trace.push(loc.clone(), format!("{}", exprs[0]))),
        },
        ast::Expression::Block(exprs) => Ok((ValRef::Block(exprs.clone()), scope)),
    }?;

    loop {
        match val {
            ValRef::Lazy(lazy) => (val, scope) = resolve_lazy(&lazy, scope)?,
            ValRef::ProtectedLazy(lazy) => return Ok((ValRef::Lazy(lazy), scope)),
            _ => return Ok((val, scope)),
        }
    }
}

pub fn eval_multiple(exprs: &[ast::Expression], scope: Scope) -> FuncResult {
    let (mut retval, mut scope) = (ValRef::None, scope);
    for expr in exprs {
        drop(retval);
        (retval, scope) = eval(expr, scope)?;
    }

    Ok((retval, scope))
}
