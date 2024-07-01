use crate::{
    lexer::{token::Token, Lexer},
    parser::r#type::Type,
};

use super::{
    super::{expression::Expression, parse_expression},
    r#type::parse_type,
};

/// Called after Token::LeftParenCurly
pub fn parse_variable_definition(
    lexer: &mut Lexer,
    start_token: Token,
) -> Result<Expression, String> {
    let name = lexer.next();

    let name = match name {
        Some(Token::ValueIdentifier(identifier)) => identifier,

        _ => return Err(format!("Variable name should be an identifier")),
    };

    let mut type_: Option<Box<Type>> = None;

    if lexer.peek() != Some(Token::Equals) {
        type_ = Some(Box::new(parse_type(lexer)?));
    }

    lexer.next();

    let value = Box::new(parse_expression(lexer)?);

    Ok(Expression::VariableDefinition {
        constant: start_token == Token::Val,
        name,
        value,
        type_,
    })
}
