use super::bstring::BString;
use super::eval::{eval, FuncArgs, Scope, StackTrace, ValRef};
use super::parse;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

pub enum ImportResult {
    Err(StackTrace),
    ValRef(ValRef),
    Code(PathBuf, BString),
}

pub trait Import {
    fn import(&self, ctx: &ImportCtx, name: &BString) -> ImportResult;
    fn insert(&mut self, path: BString, val: ValRef);
}

pub struct ImportCtx {
    pub importer: Rc<RefCell<dyn Import>>,
    pub cwd: BString,
}

impl ImportCtx {
    fn new(importer: Rc<RefCell<dyn Import>>, cwd: BString) -> Self {
        Self { importer, cwd }
    }
}

pub struct DefaultImporter {
    cache: HashMap<BString, ValRef>,
    builtins: HashMap<BString, ValRef>,
}

impl DefaultImporter {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            builtins: HashMap::new(),
        }
    }

    pub fn add_builtin(&mut self, name: BString, val: ValRef) {
        self.builtins.insert(name, val);
    }
}

impl Import for DefaultImporter {
    fn import(&self, ctx: &ImportCtx, name: &BString) -> ImportResult {
        if let Some(val) = self.builtins.get(name) {
            return ImportResult::ValRef(val.clone());
        }

        let path: PathBuf;
        if name.starts_with(b"/") {
            path = name.to_path();
        } else {
            path = ctx.cwd.to_path().join(name.to_path());
        }

        let abspath = match fs::canonicalize(path) {
            Ok(path) => path,
            Err(err) => return ImportResult::Err(StackTrace::from_string(err.to_string())),
        };

        let code = match fs::read(&abspath) {
            Ok(code) => BString::from_vec(code),
            Err(err) => return ImportResult::Err(StackTrace::from_string(err.to_string())),
        };

        ImportResult::Code(abspath, code)
    }

    fn insert(&mut self, path: BString, val: ValRef) {
        self.cache.insert(path, val);
    }
}

fn import(
    ctx: &Rc<ImportCtx>,
    name: &BString,
    scope: &Rc<RefCell<Scope>>,
) -> Result<ValRef, StackTrace> {
    let (abspath, code) = match ctx.importer.borrow().import(ctx, name) {
        ImportResult::Err(err) => return Err(err),
        ImportResult::ValRef(val) => return Ok(val),
        ImportResult::Code(path, code) => (path, code),
    };

    let mut dirpath = abspath.clone();
    dirpath.pop();

    let scope = Rc::new(RefCell::new(Scope::new_with_parent(scope.clone())));

    let childctx = Rc::new(ImportCtx::new(
        ctx.importer.clone(),
        BString::from_os_str(dirpath.as_os_str()),
    ));
    init_with_importer(&scope, childctx);

    let mut reader =
        parse::Reader::new(&code.as_bytes(), BString::from_os_str(abspath.as_os_str()));

    let mut retval = ValRef::None;
    loop {
        let expr = match parse::parse(&mut reader) {
            Ok(expr) => match expr {
                Some(expr) => expr,
                None => break,
            },
            Err(err) => {
                return Err(StackTrace::from_string(format!(
                    "{}: Parse error: {}:{}: {}",
                    name, err.line, err.col, err.msg
                )))
            }
        };

        drop(retval);
        match eval(&expr, &scope) {
            Ok(val) => retval = val,
            Err(err) => return Err(err),
        }
    }

    ctx.importer
        .borrow_mut()
        .insert(BString::from_os_str(abspath.as_os_str()), retval.clone());

    Ok(retval)
}

fn lib_import(
    importctx: &Rc<ImportCtx>,
    mut args: Vec<ValRef>,
    scope: &Rc<RefCell<Scope>>,
) -> Result<ValRef, StackTrace> {
    let mut args = args.drain(0..);

    let path = args.next_val()?.get_string()?;
    args.done()?;

    import(importctx, path.as_ref(), scope)
}

pub fn init_with_importer(scope: &Rc<RefCell<Scope>>, ctx: Rc<ImportCtx>) {
    let mut s = scope.borrow_mut();
    let c = ctx.clone();
    s.put_func("import", Rc::new(move |a, s| lib_import(&c, a, s)));
}

pub fn init_with_cwd(scope: &Rc<RefCell<Scope>>, cwd: BString) {
    init_with_importer(
        scope,
        Rc::new(ImportCtx::new(
            Rc::new(RefCell::new(DefaultImporter::new())),
            cwd,
        )),
    )
}

pub fn init_with_path(scope: &Rc<RefCell<Scope>>, path: BString) {
    let mut dirpath = path.to_path();
    dirpath.pop();
    init_with_cwd(scope, BString::from_os_str(dirpath.as_os_str()));
}
