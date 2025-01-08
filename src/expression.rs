pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[derive(Debug)]
pub enum Expression {
    Integer(i32),
    Addition(Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum ExpressionError {
    InvalidTokens,
}
