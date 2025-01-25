use pest::Parser;
use pest::RuleType;
use pest_derive::Parser;
use pest::iterators::Pair;
use pest::iterators::Pairs;

use crate::expression::Expression;

#[derive(Parser)]
#[grammar = "src/reader/lexer/math.pest"]
pub struct MathParser;


pub fn pest_to_expression(input: Pair<Rule>) -> Expression {
    let mut inner = input.clone().into_inner();
    match inner.clone().count() {
        0 => atomic_expr(input.as_rule(), input.as_str()),
        1 => pest_to_expression(inner.next().unwrap()),
        _ => compound_expr(input)
    }
}

fn atomic_expr<Rule: RuleType>(rule: Rule, content: &str) -> Expression {
    Expression::Number(0)
}

fn compound_expr(input: Pair<Rule>) -> Expression {
    println!("compound expression: {:}", input);
    Expression::Number(0)
}

#[cfg(test)]
mod tests {

use super::*;
    #[test]
    fn test_parse1() {
        let successful_parse = MathParser::parse(Rule::math, "1 + 3");
        println!("{:#?}\n\n\n", successful_parse);
        println!("{:#?}", pest_to_expression(successful_parse.unwrap().next().unwrap()));
    }
}

// TODO: make recursive function to go thru tree and map to a struct of structs that will be
// a nicer representation of the expression.
