use super::eval::{ValRef, Scope};

use std::rc::Rc;

fn lib_print(args: &Vec<ValRef>) -> ValRef {
    for idx in 0..args.len() {
        if idx != 0 {
            print!(" ");
        }
        print!("{}", args[idx]);
    }
    println!();
    ValRef::None
}

pub fn new(parent: Option<Rc<Scope>>) -> Scope {
    let mut scope = Scope::new(parent);

    let mut put = |name: &str, func: &'static dyn Fn(&Vec<ValRef>) -> ValRef | {
        scope.insert(name.to_string(), ValRef::Func(func));
    };

    put("print", &lib_print);

    scope
}
