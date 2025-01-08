
pub(super) mod lexer;
pub(super) mod parser;

use crate::expression::*;
use crate::expression::Expression::*;

pub fn initialize() {
    lexer::initialize();
    parser::initialize();
}

pub fn read(input: String) -> Result<Expression, ExpressionError> {
    let tokens = lexer::lex(input);
    Ok(Integer(0))
}
