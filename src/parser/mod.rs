use expression::Expression;
use parse_functions::function_arguments::parse_function_arguments;
use parse_functions::function_definition::parse_function_definition;
use parse_functions::if_condition::parse_if_condition;
use parse_functions::variable_definition::parse_variable_definition;
use parser_error::ParserError;
use util::token_matches;

use crate::lexer::token::TokenData;
use crate::lexer::{token::Token, Lexer};
use crate::parser::parse_functions::block::parse_block;

pub mod expression;
mod parse_functions;
pub mod r#type;
mod util;

#[macro_use]
pub mod parser_error;

pub fn parse_source(lexer: &mut Lexer) -> Result<Expression, ParserError> {
    let mut expressions = vec![];

    loop {
        let token = lexer.peek();
        if token.is_some() {
            let expr = parse_expression(lexer)?;
            expressions.push(expr.clone());

            let mut require_semicolon = true;

            match expr {
                // semicolons aren't required after top level function
                // definitions
                Expression::FunctionDefinition { name, .. } => {
                    if name.is_none() {
                        return Err(parser_error!(
                            token.unwrap().line,
                            "Top level function definitions require a name"
                        ));
                    }

                    require_semicolon = false;
                }

                _ => {}
            }

            if require_semicolon {
                let next = lexer.next();

                if !matches!(
                    next,
                    Some(Token {
                        data: TokenData::Semicolon,
                        ..
                    })
                ) {
                    if let Some(token) = next {
                        return Err(parser_error!(
                            token.line,
                            "Expected semicolon, got {:?}",
                            token.data
                        ));
                    }
                }
            }
        } else {
            break;
        }
    }

    Ok(Expression::Block { expressions })
}

fn parse_expression(lexer: &mut Lexer) -> Result<Expression, ParserError> {
    expr_binding_power(lexer, 0)
}

fn expr_binding_power(
    lexer: &mut Lexer,
    min_binding_power: u8,
) -> Result<Expression, ParserError> {
    let mut left = match lexer.next() {
        Some(Token {
            data: TokenData::ValueString(v),
            ..
        }) => Expression::String(v),
        Some(Token {
            data: TokenData::ValueChar(v),
            ..
        }) => Expression::Char(v),
        Some(Token {
            data: TokenData::ValueInt(v),
            ..
        }) => Expression::Int(v),
        Some(Token {
            data: TokenData::ValueFloat(v),
            ..
        }) => Expression::Float(v),
        Some(Token {
            data: TokenData::ValueIdentifier(v),
            ..
        }) => Expression::Identifier(v),

        Some(
            operator @ Token {
                data: TokenData::Minus,
                ..
            },
        ) => {
            let (_, right_binding_power) =
                prefix_binding_power(&operator.data).unwrap();
            let right = expr_binding_power(lexer, right_binding_power)?;

            Expression::Prefix {
                operator: operator.data,
                value: Box::new(right),
            }
        }

        Some(Token {
            data: TokenData::LeftParenNormal,
            ..
        }) => {
            let inner_expression = parse_expression(lexer);

            let next = lexer.next();
            if !token_matches(&next, &TokenData::RightParenNormal) {
                return Err(parser_error!(next.unwrap().line, "Expected ("));
            }

            inner_expression?
        }

        Some(Token {
            data: TokenData::LeftParenCurly,
            ..
        }) => parse_block(lexer)?,

        Some(Token {
            data: TokenData::If,
            ..
        }) => parse_if_condition(lexer)?,

        token @ Some(Token {
            data: TokenData::Val | TokenData::Var,
            ..
        }) => parse_variable_definition(lexer, token.unwrap())?,

        Some(Token {
            data: TokenData::Fun,
            ..
        }) => parse_function_definition(lexer)?,

        None => return Err(parser_error_eof!("Expected expression")),
        Some(t) => {
            return Err(parser_error!(t.line, "Unexpected token: {:?}", t.data))
        }
    };

    loop {
        let operator = match lexer.peek() {
            Some(token) => token,

            _ => break,
        };

        match postfix_binding_power(&operator.data) {
            Some((left_binding_power, ())) => {
                if left_binding_power < min_binding_power {
                    break;
                }

                lexer.next();

                left = match operator.data {
                    // function call
                    TokenData::LeftParenNormal => Expression::FunctionCall {
                        function: Box::new(left),
                        arguments: parse_function_arguments(lexer)?,
                    },

                    // else
                    _ => Expression::Postfix {
                        value: Box::new(left),
                        operator: operator.data,
                    },
                };

                continue;
            }

            _ => {}
        }

        match infix_binding_power(&operator.data) {
            Some((left_binding_power, right_binding_power)) => {
                if left_binding_power < min_binding_power {
                    break;
                }

                lexer.next();
                let right = expr_binding_power(lexer, right_binding_power)?;

                left = Expression::Infix {
                    left: Box::new(left),
                    operator: operator.data,
                    right: Box::new(right),
                };

                continue;
            }

            _ => {}
        }

        break;
    }

    Ok(left)
}

fn prefix_binding_power(op: &TokenData) -> Option<((), u8)> {
    match op {
        TokenData::Minus => Some(((), 30)),

        _ => None,
    }
}

fn infix_binding_power(op: &TokenData) -> Option<(u8, u8)> {
    match op {
        TokenData::Plus | TokenData::Minus => Some((10, 11)),
        TokenData::Star | TokenData::Slash => Some((20, 21)),

        TokenData::EqualsEquals
        | TokenData::LessThan
        | TokenData::LessThanOrEqual
        | TokenData::GreaterThan
        | TokenData::GreaterThanOrEqual => Some((0, 1)),

        _ => None,
    }
}

fn postfix_binding_power(op: &TokenData) -> Option<(u8, ())> {
    match op {
        TokenData::LeftParenNormal => Some((40, ())),
        _ => None,
    }
}
