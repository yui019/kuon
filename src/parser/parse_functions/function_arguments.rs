use crate::{
    lexer::{
        token::{Token, TokenData},
        Lexer,
    },
    parser::parser_error::ParserError,
    parser_error, parser_error_eof, token_data,
};

use super::super::{expression::Expression, parse_expression};

/// Called after Token::LeftParenNormal
pub fn parse_function_arguments(
    lexer: &mut Lexer,
) -> Result<Vec<Expression>, ParserError> {
    let mut arguments: Vec<Expression> = vec![];

    // immediately return if no arguments
    if matches!(lexer.peek(), Some(token_data!(TokenData::RightParenNormal))) {
        lexer.next();

        return Ok(arguments);
    }

    loop {
        let argument = parse_expression(lexer)?;
        arguments.push(argument);

        match lexer.peek() {
            Some(token_data!(TokenData::RightParenNormal)) => {
                lexer.next();
                break;
            }

            Some(token_data!(TokenData::Comma)) => {
                lexer.next();
                continue;
            }

            Some(t) => {
                return Err(parser_error!(
                    t.line,
                    "Expected , or ), got {:?}",
                    t.data
                ))
            }
            None => return Err(parser_error_eof!("Expected , or )")),
        }
    }

    Ok(arguments)
}
