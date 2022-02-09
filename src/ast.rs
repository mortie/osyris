use std::rc::Rc;

#[derive(Debug)]
pub enum Expression {
    String(String),
    Number(i32),
    Name(String),
    Call(Vec<Expression>),
    Quote(Rc<Vec<Expression>>),
}
