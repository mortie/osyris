use super::eval::{Scope, ValRef, PortVal};
use std::cell::RefCell;
use std::rc::Rc;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::io::Seek;

struct TextFile {
    f: fs::File,
}

impl PortVal for TextFile {
    fn read(&mut self) -> Result<ValRef, String> {
        let mut buf = String::new();
        match self.f.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string()),
        };

        Ok(ValRef::String(Rc::new(buf)))
    }

    fn write(&mut self, val: &ValRef) -> Result<(), String> {
        let res = match val {
            ValRef::String(s) => self.f.write(s.as_bytes()),
            val => self.f.write(format!("{}", val).as_bytes()),
        };

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    fn seek(&mut self, pos: io::SeekFrom) -> Result<(), String> {
        match self.f.seek(pos) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}

pub fn lib_open(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 1 {
        return Err("'open' requires 1 argument".to_string());
    }

    let path = match &args[0] {
        ValRef::String(s) => s,
        _ => return Err("'open' requires the first argument to be a string".to_string()),
    };

    let f = match fs::File::open(path.as_ref()) {
        Ok(f) => f,
        Err(err) => return Err(format!("'open': {}: {}", path, err)),
    };

    Ok(ValRef::Port(Rc::new(RefCell::new(TextFile{f}))))
}

pub fn lib_create(args: Vec<ValRef>, _: &Rc<RefCell<Scope>>) -> Result<ValRef, String> {
    if args.len() != 1 {
        return Err("'create' requires 1 argument".to_string());
    }

    let path = match &args[0] {
        ValRef::String(s) => s,
        _ => return Err("'create' requires the first argument to be a string".to_string()),
    };

    let f = match fs::File::create(path.as_ref()) {
        Ok(f) => f,
        Err(err) => return Err(format!("'create': {}: {}", path, err)),
    };

    Ok(ValRef::Port(Rc::new(RefCell::new(TextFile{f}))))
}

pub fn init(scope: &Rc<RefCell<Scope>>) {
    scope.borrow_mut().put_func("open", Rc::new(lib_open));
    scope.borrow_mut().put_func("create", Rc::new(lib_create));
}
