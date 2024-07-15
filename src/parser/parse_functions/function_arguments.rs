use crate::{
    lexer::{
        token::{Token, TokenData},
        Lexer,
    },
    parser::parser_error::ParserError,
    parser_error, parser_error_eof, some_token_pat,
};

use super::super::{expression::Expression, parse_expression};

/// Called after Token::LeftParenNormal
pub fn parse_function_arguments(
    lexer: &mut Lexer,
) -> Result<Vec<Expression>, ParserError> {
    let mut arguments: Vec<Expression> = vec![];

    // immediately return if no arguments
    if matches!(lexer.peek(), some_token_pat!(TokenData::RightParenNormal)) {
        lexer.next();

        return Ok(arguments);
    }

    loop {
        let argument = parse_expression(lexer)?;
        arguments.push(argument);

        match lexer.peek() {
            some_token_pat!(TokenData::RightParenNormal) => {
                lexer.next();
                break;
            }

            some_token_pat!(TokenData::Comma) => {
                lexer.next();
                continue;
            }

            Some(t) => {
                return parser_error!(
                    t.line,
                    "Expected , or ), got {:?}",
                    t.data
                )
            }
            None => return parser_error_eof!("Expected , or )"),
        }
    }

    Ok(arguments)
}
