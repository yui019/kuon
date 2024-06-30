use crate::lexer::{token::Token, Lexer};

use super::super::{expression::Expression, parse_expression};

/// Called after Token::LeftParenCurly
pub fn parse_if_condition(lexer: &mut Lexer) -> Result<Expression, String> {
    let condition = Box::new(parse_expression(lexer)?);
    let true_branch = Box::new(parse_expression(lexer)?);
    let mut else_branch: Option<Box<Expression>> = None;

    if lexer.peek() == Some(Token::Else) {
        lexer.next();
        else_branch = Some(Box::new(parse_expression(lexer)?));
    }

    Ok(Expression::IfCondition {
        condition,
        true_branch,
        else_branch,
    })
}
