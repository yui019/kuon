use crate::lexer::{token::Token, Lexer};

use super::super::{expression::Expression, parse_expression};

/// Called after Token::LeftParenCurly
pub fn parse_variable_definition(
    lexer: &mut Lexer,
    start_token: Token,
) -> Result<Expression, String> {
    let name = lexer.next();

    if !matches!(name, Some(Token::ValueIdentifier(_))) {
        return Err(format!("Variable name should be an identifier"));
    }

    let name = name.unwrap();

    if lexer.next() != Some(Token::Equals) {
        return Err(format!("Expected = after variable name"));
    }

    let value = Box::new(parse_expression(lexer)?);

    Ok(Expression::VariableDefinition {
        constant: start_token == Token::Val,
        name,
        value,
    })
}
