use super::bstring::BString;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Location {
    pub line: u32,
    pub column: u32,
    pub file: Rc<BString>,
}

#[derive(Debug)]
pub enum Expression {
    String(BString),
    Number(f64),
    Lookup(BString),
    Call(Vec<Expression>, Location),
    Block(Rc<Vec<Expression>>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::String(s) => {
                write!(f, "{:?}", s)
            }
            Expression::Number(num) => write!(f, "{}", num),
            Expression::Lookup(name) => write!(f, "{}", name),
            Expression::Call(exprs, _) => {
                write!(f, "(")?;
                let mut first = true;
                for expr in exprs {
                    if !first {
                        write!(f, " ")?;
                    }
                    first = false;

                    write!(f, "{}", expr)?;
                }
                write!(f, ")")
            }
            Expression::Block(exprs) => {
                write!(f, "'(")?;
                let mut first = true;
                for expr in exprs.iter() {
                    if !first {
                        write!(f, " ")?;
                    }
                    first = false;

                    write!(f, "{}", expr)?;
                }
                write!(f, ")")
            }
        }
    }
}
