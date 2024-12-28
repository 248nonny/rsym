use regex::Regex;
use super::expression::Expression;

pub fn string_to_expression(input: String) -> Expression {
    let result = parse(input);

    match result {
        Ok(expr) => return expr,
        Err(err) => panic!("{} encountered, exiting program.",format!("{err:?}")),
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidInputError,
    //ParseError(String),
}

/**
 * An enum for describing parse rules; a master rule array is
 * created later, and is iterated over to define an order of rule precedence.
 * 
 * A Unary rule is one which will only refer to one expression; for example,
 * parsing brackets is unary (you just group the stuff in the brackets into a
 * single expression), and a function (e.g. sin(...)) is also unary, since
 * there is only one sub-expression to parse.
 *
 * A Binary rule is one which will have two sub expressions; all binary operations
 * (addition, sub., mult., div) are binary (duh). This could also apply to higher
 * order functions (kroenecker delta??).
 */
enum ParserRule {
    /**
     * left and right are regex patterns which match the edges of the rule domain
     * (i.e. the part of the string that the rule should apply to). If body
     * is defined, then only the part of the substring that matches body will 
     * be further parsed to extract the child expression.
     */
    Unary{left: String, right: String, body: Option<String>};
    Binary{left: String, center: String, right: String}
}

fn parse(input: String) -> Result<Expression, ParseError> {
    if input.is_empty() {
        return Err(ParseError::InvalidInputError);
    }

    Ok(Expression::Integer(0))
}
