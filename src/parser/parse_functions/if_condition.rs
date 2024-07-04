use crate::{
    lexer::{token::TokenData, Lexer},
    parser::parser_error::ParserError,
};

use super::super::{
    expression::Expression, parse_expression, util::token_matches,
};

/// Called after Token::LeftParenCurly
pub fn parse_if_condition(
    lexer: &mut Lexer,
) -> Result<Expression, ParserError> {
    let condition = Box::new(parse_expression(lexer)?);
    let true_branch = Box::new(parse_expression(lexer)?);
    let mut else_branch: Option<Box<Expression>> = None;

    if token_matches(&lexer.peek(), &TokenData::Else) {
        lexer.next();
        else_branch = Some(Box::new(parse_expression(lexer)?));
    }

    Ok(Expression::IfCondition {
        condition,
        true_branch,
        else_branch,
    })
}
