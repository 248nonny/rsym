pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[derive(Debug)]
pub enum Expression {
    Number(i64),
    BOp(Box<BOpType>, Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum BOpType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent(Box<Base>)
}

#[derive(Debug)]
pub enum Base {
    Number(Box<Expression>)
}

#[derive(Debug)]
pub enum ExpressionError {
    InvalidTokens,
}
