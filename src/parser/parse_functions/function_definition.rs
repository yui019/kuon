use crate::{
    expression,
    lexer::{
        token::{Token, TokenData},
        Lexer,
    },
    parser::{
        expression::FunctionParam, parse_expression, parser_error::ParserError,
        util::token_matches,
    },
    parser_error, parser_error_eof, some_token_pat,
};

use super::{super::expression::Expression, r#type::parse_type};

pub fn parse_params(
    lexer: &mut Lexer,
) -> Result<Vec<FunctionParam>, ParserError> {
    let mut params: Vec<FunctionParam> = vec![];

    loop {
        // determine if parameter is constant based on the optional val/var
        let constant = if matches!(
            &lexer.peek(),
            some_token_pat!(TokenData::Val | TokenData::Var)
        ) {
            matches!(&lexer.next(), some_token_pat!(TokenData::Val))
        } else {
            true
        };

        let next = lexer.next();

        if token_matches(&next, &TokenData::RightParenNormal) {
            break;
        }

        match next {
            some_token_pat!(TokenData::ValueIdentifier(identifier)) => {
                let name = identifier;
                let type_ = parse_type(lexer)?;

                params.push(FunctionParam {
                    name,
                    type_,
                    constant,
                });

                match lexer.next() {
                    some_token_pat!(TokenData::Comma) => {}

                    some_token_pat!(TokenData::RightParenNormal) => break,

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

            Some(t) => {
                return parser_error!(
                    t.line,
                    "Expected identifier, got {:?}",
                    t.data
                )
            }
            None => return parser_error_eof!("Expected parameter"),
        }
    }

    return Ok(params);
}

/// Called after Token::Fun
pub fn parse_function_definition(
    lexer: &mut Lexer,
    top_level: bool,
    line: usize,
) -> Result<Expression, ParserError> {
    let mut name: Option<String> = None;

    // get name if it exists
    let first_token = lexer.next();
    match first_token.clone() {
        some_token_pat!(TokenData::LeftParenNormal) => {}

        some_token_pat!(TokenData::ValueIdentifier(identifier)) => {
            name = Some(identifier);

            let next = lexer.next();
            if !token_matches(&next, &TokenData::LeftParenNormal) {
                return parser_error!(
                    next.unwrap().line,
                    "Expected ( after function name",
                );
            }
        }

        Some(t) => {
            return parser_error!(
                t.line,
                "Expected function name or (, got {:?}",
                t.data
            )
        }
        None => return parser_error_eof!("Expected function name or ("),
    }

    if !top_level && name.is_some() {
        return parser_error!(
            first_token.unwrap().line,
            "Only top level functions can have a name",
        );
    }

    let params = parse_params(lexer)?;

    if matches!(lexer.peek(), some_token_pat!(TokenData::Colon)) {
        if params.len() != 1 {
            return parser_error!(
                first_token.unwrap().line,
                "There can only be one function pre-parameter",
            );
        }

        if !top_level {
            return parser_error!(
                first_token.unwrap().line,
                "Only top level functions can have a pre-parameter",
            );
        }

        if name.is_some() {
            return parser_error!(
                first_token.unwrap().line,
                "The function name goes after the pre-parameter",
            );
        }

        lexer.next();

        let pre_parameter = params[0].clone();

        let name = match lexer.next() {
            some_token_pat!(TokenData::ValueIdentifier(n)) => n,

            Some(t) => {
                return parser_error!(
                    t.line,
                    "Expected function name, got {:?}",
                    t.data
                )
            }
            None => return parser_error_eof!("Expected function name"),
        };

        match lexer.next() {
            some_token_pat!(TokenData::LeftParenNormal) => {}

            Some(t) => {
                return parser_error!(t.line, "Expected (, got {:?}", t.data)
            }
            None => return parser_error_eof!("Expected ("),
        };

        let params = parse_params(lexer)?;
        let return_type = parse_type(lexer)?;
        let body = parse_expression(lexer)?;

        Ok(expression!(
            FunctionDefinition {
                name: Some(name),
                pre_parameter: Some(pre_parameter),
                params,
                return_type,
                body: Box::new(body),
            },
            line
        ))
    } else {
        let return_type = parse_type(lexer)?;
        let body = parse_expression(lexer)?;

        Ok(expression!(
            FunctionDefinition {
                name,
                pre_parameter: None,
                params,
                return_type,
                body: Box::new(body),
            },
            line
        ))
    }
}
