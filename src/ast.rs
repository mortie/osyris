use std::rc::Rc;

#[derive(Debug)]
pub enum Expression {
    String(String),
    Number(f64),
    Lookup(String),
    Call(Vec<Expression>),
    Quote(Rc<Vec<Expression>>),
}
