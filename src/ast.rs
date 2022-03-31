use super::bstring::BString;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expression {
    String(BString),
    Number(f64),
    Lookup(BString),
    Call(Vec<Expression>),
    Quote(Rc<Vec<Expression>>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::String(s) => {
                write!(f, "\"")?;
                for ch in s {
                    if *ch == b'\\' {
                        write!(f, "\\\\")?;
                    } else if *ch == b'"' {
                        write!(f, "\\\"")?;
                    } else {
                        write!(f, "{}", ch)?;
                    }
                }
                write!(f, "\"")
            }
            Expression::Number(num) => write!(f, "{}", num),
            Expression::Lookup(name) => write!(f, "{}", name),
            Expression::Call(exprs) => {
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
            Expression::Quote(exprs) => {
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
