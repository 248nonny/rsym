pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub enum Expression {
    Integer(i32),
    Addition(Box<Expression>, Box<Expression>),
}
