use super::eval::{Scope, StackTrace, ValRef};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

fn write_val<W>(w: &mut W, val: &ValRef, parent: String) -> Result<String, io::Error>
where
    W: io::Write,
{
    let name;
    match val {
        ValRef::None => {
            name = parent;
            write!(w, "{} [label=\"None\" shape=box]\n", name)?;
        }
        ValRef::Number(num) => {
            name = parent;
            write!(w, "{} [label=\"{}\" shape=box]\n", name, num)?;
        }
        ValRef::Bool(b) => {
            name = parent;
            write!(w, "{} [label=\"{}\" shape=box]\n", name, b)?;
        }
        ValRef::String(s) => {
            name = format!("v{:p}", s.as_ref());
            write!(
                w,
                "{} [label=\"string rc={}\"]\n",
                name,
                Rc::strong_count(s)
            )?;
            write!(w, "{}c [label={:?} shape=box]\n", name, s.as_ref())?;
            write!(w, "{} -> {}c [label=\"::content\"]\n", name, name)?;
        }
        ValRef::Block(b) => {
            name = format!("v{:p}", b.as_ref());
            write!(w, "{} [label=\"block rc={}\"]\n", name, Rc::strong_count(b))?;
        }
        ValRef::List(l) => {
            name = format!("v{:p}", l.as_ref());
            write!(w, "{} [label=\"list rc={}\"]\n", name, Rc::strong_count(l))?;

            let vec = l.borrow();
            for idx in 0..vec.len() {
                let n = write_val(w, &vec[idx], format!("{}v{}", name, idx))?;
                write!(w, "{} -> {} [label=\"[{}]\"]\n", name, n, idx)?;
            }
        }
        ValRef::Dict(d) => {
            name = format!("v{:p}", d.as_ref());
            write!(w, "{} [label=\"dict rc={}\"]\n", name, Rc::strong_count(d))?;

            let map = d.borrow();
            let mut idx = 0;
            for (key, val) in map.iter() {
                let n = write_val(w, &val, format!("{}v{}", name, idx))?;
                write!(w, "{} -> {} [label={:?}]\n", name, n, key)?;
                idx += 1;
            }
        }
        ValRef::Func(f) => {
            name = format!("v{:p}", f.as_ref());
            write!(w, "{} [label=\"func rc={}\"]\n", name, Rc::strong_count(f))?;
        }
        ValRef::Lambda(l) => {
            name = format!("v{:p}", l.as_ref());
            write!(
                w,
                "{} [label=\"lambda rc={}\"]\n",
                name,
                Rc::strong_count(l)
            )?;
        }
        ValRef::Binding(b, func) => {
            name = format!("{}", parent);
            write!(w, "{} [label=\"binding\"]\n", name)?;

            let mut idx = 0;
            for (key, val) in b.as_ref() {
                let n = write_val(w, &val, format!("{}v{}", name, idx))?;
                write!(w, "{} -> {} [label={:?}]\n", name, n, key)?;
                idx += 1;
            }

            let n = write_val(w, func.as_ref(), format!("{}f", name))?;
            write!(w, "{} -> {} [label=\"::func\"]\n", name, n)?;
        }
        ValRef::Lazy(l) => {
            name = format!("v{:p}", l.as_ref());
            write!(w, "{} [label=\"lazy rc={}\"]\n", name, Rc::strong_count(l))?;
        }
        ValRef::ProtectedLazy(p) => {
            name = format!("v{:p}", p.as_ref());
            let lname = write_val(w, p.as_ref(), format!("{}l", name))?;
            write!(w, "{} [label=\"protected lazy\"]\n", name)?;
            write!(w, "{} -> {} [label=\"::lazy\"]\n", name, lname)?;
        }
        ValRef::Native(n) => {
            name = format!("v{:p}", n.as_ref());
            write!(
                w,
                "{} [label=\"native rc={}\"]\n",
                name,
                Rc::strong_count(n)
            )?;
        }
        ValRef::Port(p) => {
            name = format!("v{:p}", p.as_ref());
            write!(w, "{} [label=\"port rc={}\"]\n", name, Rc::strong_count(p))?;
        }
    }

    Ok(name)
}

fn write_scope<W>(w: &mut W, scope: &Rc<RefCell<Scope>>) -> Result<(), io::Error>
where
    W: io::Write,
{
    write!(w, "s{:p} [label=\"scope\"]\n", scope.as_ref())?;

    let s = scope.borrow();
    let mut idx = 0;
    for (key, val) in &s.map {
        let name = write_val(w, val, format!("s{:p}v{}", scope.as_ref(), idx))?;
        write!(
            w,
            "s{:p} -> {} [label={:?} type=s]\n",
            scope.as_ref(),
            name,
            key
        )?;
        idx += 1;
    }

    match &scope.borrow().parent {
        None => (),
        Some(parent) => {
            if parent.borrow().parent.is_some() {
                write_scope(w, parent)?;
                write!(
                    w,
                    "s{:p} -> s{:p} [label=\"::parent\"]\n",
                    scope.as_ref(),
                    parent.as_ref()
                )?;
            }
        }
    };

    Ok(())
}

pub fn write_dot<W>(w: &mut W, scope: &Rc<RefCell<Scope>>) -> Result<(), io::Error>
where
    W: io::Write,
{
    write!(w, "digraph d {{\n")?;
    write_scope(w, scope)?;
    write!(w, "}}\n")
}

fn lib_print_scope_dot(_: Vec<ValRef>, scope: &Rc<RefCell<Scope>>) -> Result<ValRef, StackTrace> {
    match write_dot(&mut io::stdout(), scope) {
        Ok(()) => Ok(ValRef::None),
        Err(err) => Err(StackTrace::from_string(err.to_string())),
    }
}

pub fn init(scope: &Rc<RefCell<Scope>>) {
    let mut s = scope.borrow_mut();
    s.put_func("print-scope-dot", Rc::new(lib_print_scope_dot));
}
