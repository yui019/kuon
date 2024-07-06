use crate::{
    lexer::{
        token::{Token, TokenData},
        Lexer,
    },
    parser::parser_error::ParserError,
    parser_error, parser_error_eof, token_data,
};

use super::super::{
    expression::Expression, parse_expression, util::token_matches,
};

/// Called after Token::LeftParenCurly
pub fn parse_block(lexer: &mut Lexer) -> Result<Expression, ParserError> {
    let mut expressions: Vec<Expression> = vec![];

    if token_matches(&lexer.peek(), &TokenData::RightParenCurly) {
        lexer.next();
        expressions.push(Expression::Null);

        return Ok(Expression::Block { expressions });
    }

    loop {
        expressions.push(parse_expression(lexer)?);

        match lexer.next() {
            Some(token_data!(TokenData::Semicolon)) => {
                if token_matches(&lexer.peek(), &TokenData::RightParenCurly) {
                    lexer.next();
                    expressions.push(Expression::Null);
                    break;
                }

                continue;
            }

            Some(token_data!(TokenData::RightParenCurly)) => break,

            Some(t) => {
                return Err(parser_error!(
                    t.line,
                    "Expected semicolon inside block, got {:?}",
                    t.data
                ))
            }
            None => return Err(parser_error_eof!("Expected }}")),
        }
    }

    Ok(Expression::Block { expressions })
}
