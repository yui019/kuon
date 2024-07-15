use crate::{
    expression,
    lexer::{
        token::{Token, TokenData::*},
        Lexer,
    },
    parser::{parser_error::ParserError, r#type::Type},
    parser_error, parser_error_eof, some_token_pat,
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
        some_token_pat!(ValueIdentifier(identifier)) => identifier,

        None => return parser_error_eof!("Expected variable name"),
        Some(t) => {
            return parser_error!(
                t.line,
                "Variable name should be an identifier"
            )
        }
    };

    let mut type_: Option<Box<Type>> = None;

    if !token_matches(&lexer.peek(), &Equals) {
        type_ = Some(Box::new(parse_type(lexer)?));
    }

    lexer.next();

    let value = Box::new(parse_expression(lexer)?);

    Ok(expression!(
        VariableDefinition {
            constant: start_token.data == Val,
            name,
            value,
            type_,
        },
        start_token.line
    ))
}
