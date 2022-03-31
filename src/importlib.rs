use super::bstring::BString;
use super::eval::{eval, Scope, ValRef};
use super::parse;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

pub trait Import {
    fn import(&mut self, ctx: &ImportCtx, name: &BString) -> Result<ValRef, String>;
}

pub struct ImportCtx {
    pub importer: Rc<RefCell<dyn Import>>,
    pub cwd: BString,
    pub root_scope: Rc<RefCell<Scope>>,
}

impl ImportCtx {
    fn new(
        importer: Rc<RefCell<dyn Import>>,
        cwd: BString,
        root_scope: Rc<RefCell<Scope>>,
    ) -> Self {
        Self {
            importer,
            cwd,
            root_scope,
        }
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
    fn import(&mut self, ctx: &ImportCtx, name: &BString) -> Result<ValRef, String> {
        if let Some(val) = self.builtins.get(name) {
            return Ok(val.clone());
        }

        let path: PathBuf;
        if name.starts_with(b"/") {
            path = name.to_path();
        } else {
            path = ctx.cwd.to_path().join(name.to_path());
        }

        let abspath = match fs::canonicalize(path) {
            Ok(path) => path,
            Err(err) => return Err(err.to_string()),
        };

        let pathstr = BString::from_os_str(abspath.as_os_str());

        let mut absdir = abspath.clone();
        absdir.pop();

        let dirstr = BString::from_os_str(absdir.as_os_str());

        if let Some(val) = self.cache.get(&dirstr) {
            return Ok(val.clone());
        }

        let code = match fs::read_to_string(&abspath) {
            Ok(code) => code,
            Err(err) => return Err(err.to_string()),
        };

        let scope = Rc::new(RefCell::new(Scope::new_with_parent(ctx.root_scope.clone())));

        let childctx = Rc::new(ImportCtx::new(
            ctx.importer.clone(),
            dirstr,
            ctx.root_scope.clone(),
        ));
        init_with_importer(&scope, childctx);

        let mut reader = parse::Reader::new(&code.as_bytes());

        let mut ret = ValRef::None;
        loop {
            let expr = match parse::parse(&mut reader) {
                Ok(expr) => match expr {
                    Some(expr) => expr,
                    None => break,
                },
                Err(err) => {
                    return Err(format!(
                        "{}: Parse error: {}:{}: {}",
                        name, err.line, err.col, err.msg
                    ))
                }
            };

            match eval(&expr, &scope) {
                Ok(val) => ret = val,
                Err(err) => return Err(err),
            }
        }

        self.cache.insert(pathstr, ret.clone());

        Ok(ret)
    }
}

fn import(importctx: &Rc<ImportCtx>, name: &BString) -> Result<ValRef, String> {
    match importctx.importer.borrow_mut().import(importctx, name) {
        Ok(val) => Ok(val.clone()),
        Err(err) => Err(err),
    }
}

fn lib_import(
    importctx: &Rc<ImportCtx>,
    args: Vec<ValRef>,
    _: &Rc<RefCell<Scope>>,
) -> Result<ValRef, String> {
    if args.len() != 1 {
        return Err("'import' requires 1 argument".to_string());
    }

    let path = match &args[0] {
        ValRef::String(s) => s,
        _ => return Err("'import' requires the first argument to be a string".to_string()),
    };

    import(importctx, path)
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
            scope.clone(),
        )),
    )
}

pub fn init_with_path(scope: &Rc<RefCell<Scope>>, path: BString) {
    let mut dirpath = path.to_path();
    dirpath.pop();
    init_with_cwd(scope, BString::from_os_str(dirpath.as_os_str()));
}
