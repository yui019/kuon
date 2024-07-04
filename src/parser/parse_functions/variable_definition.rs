use crate::{
    lexer::{
        token::{Token, TokenData},
        Lexer,
    },
    parser::{parser_error::ParserError, r#type::Type},
    parser_error, parser_error_eof,
};

use super::{
    super::{expression::Expression, parse_expression, util::token_matches},
    r#type::parse_type,
};

/// Called after Token::LeftParenCurly
pub fn parse_variable_definition(
    lexer: &mut Lexer,
    start_token: Token,
) -> Result<Expression, ParserError> {
    let name = lexer.next();

    let name = match name {
        Some(Token {
            data: TokenData::ValueIdentifier(identifier),
            ..
        }) => identifier,

        None => return Err(parser_error_eof!("Expected variable name")),
        Some(t) => {
            return Err(parser_error!(
                t.line,
                "Variable name should be an identifier"
            ))
        }
    };

    let mut type_: Option<Box<Type>> = None;

    if !token_matches(&lexer.peek(), &TokenData::Equals) {
        type_ = Some(Box::new(parse_type(lexer)?));
    }

    lexer.next();

    let value = Box::new(parse_expression(lexer)?);

    Ok(Expression::VariableDefinition {
        constant: start_token.data == TokenData::Val,
        name,
        value,
        type_,
    })
}
