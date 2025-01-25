
pub mod lexer;
pub mod parser;

use crate::expression::*;
use crate::expression::Expression::*;

pub fn initialize() {
    //lexer::initialize();
    parser::initialize();
}

pub fn read(input: String) -> Result<Expression, ExpressionError> {
    Ok(Number(0))
}
