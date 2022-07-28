use crate::value::Value;

#[derive(Debug)]
pub enum Expression {
    Literal(Value),
    Grouping(Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
}

#[derive(Copy, Clone, Debug)]
pub enum UnaryOperator {
    Neg,
    Not,
}

#[derive(Copy, Clone, Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Div,
    Mul,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}
