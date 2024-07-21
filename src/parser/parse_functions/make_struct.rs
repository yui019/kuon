use std::collections::HashMap;

use crate::{
    expression,
    lexer::{
        token::{Token, TokenData},
        Lexer,
    },
    parser::{parse_expression, parser_error::ParserError},
    parser_error, parser_error_eof, some_token_pat,
};

use super::super::expression::Expression;

/// Called after Token::MkStruct (or Token::Identifier if the peeked token is
/// Token::LeftParenCurly)
pub fn parse_make_struct(
    lexer: &mut Lexer,
    line: usize,
    name: Option<String>,
) -> Result<Expression, ParserError> {
    let mut fields: HashMap<String, Expression> = HashMap::new();

    // expect {
    match lexer.next() {
        some_token_pat!(TokenData::LeftParenCurly) => {}

        Some(t) => {
            return parser_error!(t.line, "Expected {{, got {:?}", t.data)
        }
        None => return parser_error_eof!("Expected {{"),
    }

    loop {
        match lexer.next() {
            some_token_pat!(TokenData::RightParenCurly) => break,

            some_token_pat!(TokenData::ValueIdentifier(name)) => {
                // expect :
                match lexer.next() {
                    some_token_pat!(TokenData::Colon) => {}

                    Some(t) => {
                        return parser_error!(
                            t.line,
                            "Expected :, got {:?}",
                            t.data
                        )
                    }
                    None => return parser_error_eof!("Expected :"),
                }

                let value = parse_expression(lexer)?;

                // skip optional comma
                if matches!(lexer.peek(), some_token_pat!(TokenData::Comma)) {
                    lexer.next();
                }

                fields.insert(name, value);
            }

            Some(t) => {
                return parser_error!(t.line, "Unexpected token: {:?}", t.data)
            }
            None => return parser_error_eof!("Expected identifier or }}"),
        };
    }

    Ok(expression!(MakeStruct { name, fields }, line))
}
