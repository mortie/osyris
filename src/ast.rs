use std::rc::Rc;
use std::fmt;

#[derive(Debug)]
pub enum Expression {
    String(String),
    Number(f64),
    Lookup(String),
    Call(Vec<Expression>),
    Quote(Rc<Vec<Expression>>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::String(s) => {
                write!(f, "\"")?;
                for ch in s.chars() {
                    if ch == '\\' {
                        write!(f, "\\\\")?;
                    } else if ch == '"' {
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
