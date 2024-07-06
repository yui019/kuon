use crate::{
    lexer::{
        token::{Token, TokenData},
        Lexer,
    },
    parser::{
        expression::FunctionParam, parse_expression, parser_error::ParserError,
        util::token_matches,
    },
    parser_error, parser_error_eof,
};

use super::{super::expression::Expression, r#type::parse_type};

pub fn parse_params(
    lexer: &mut Lexer,
) -> Result<Vec<FunctionParam>, ParserError> {
    let mut params: Vec<FunctionParam> = vec![];

    loop {
        let next = lexer.next();

        if token_matches(&next, &TokenData::RightParenNormal) {
            break;
        }

        match next {
            Some(Token {
                data: TokenData::ValueIdentifier(identifier),
                ..
            }) => {
                let name = identifier;
                let type_ = parse_type(lexer)?;

                params.push(FunctionParam { name, type_ });

                match lexer.next() {
                    Some(Token {
                        data: TokenData::Comma,
                        ..
                    }) => {}

                    Some(Token {
                        data: TokenData::RightParenNormal,
                        ..
                    }) => break,

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

            Some(t) => {
                return Err(parser_error!(
                    t.line,
                    "Expected identifier, got {:?}",
                    t.data
                ))
            }
            None => return Err(parser_error_eof!("Expected parameter")),
        }
    }

    return Ok(params);
}

/// Called after Token::Fun
pub fn parse_function_definition(
    lexer: &mut Lexer,
) -> Result<Expression, ParserError> {
    let mut name: Option<String> = None;

    // get name if it exists
    match lexer.next() {
        Some(Token {
            data: TokenData::LeftParenNormal,
            ..
        }) => {}

        Some(Token {
            data: TokenData::ValueIdentifier(identifier),
            ..
        }) => {
            name = Some(identifier);

            let next = lexer.next();
            if !token_matches(&next, &TokenData::LeftParenNormal) {
                return Err(parser_error!(
                    next.unwrap().line,
                    "Expected ( after function name",
                ));
            }
        }

        Some(t) => {
            return Err(parser_error!(
                t.line,
                "Expected function name or (, got {:?}",
                t.data
            ))
        }
        None => return Err(parser_error_eof!("Expected function name or (")),
    }

    let params = parse_params(lexer)?;
    let return_type = parse_type(lexer)?;
    let body = parse_expression(lexer)?;

    Ok(Expression::FunctionDefinition {
        name,
        params,
        return_type,
        body: Box::new(body),
    })
}
