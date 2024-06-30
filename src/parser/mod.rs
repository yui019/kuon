use expression::Expression;
use parse_functions::if_condition::parse_if_condition;

use crate::lexer::{token::Token, Lexer};
use crate::parser::parse_functions::block::parse_block;

pub mod expression;
mod parse_functions;

pub fn parse_source(lexer: &mut Lexer) -> Result<Vec<Expression>, String> {
    let mut expressions = vec![parse_expression(lexer)?];

    loop {
        match lexer.next() {
            Some(Token::Semicolon) => {
                if lexer.peek().is_some() {
                    expressions.push(parse_expression(lexer)?);
                } else {
                    expressions.push(Expression::Null);
                    break;
                }
            }

            Some(token) => {
                return Err(format!("Unexpected token: {:?}", token))
            }

            None => break,
        }
    }

    Ok(expressions)
}

fn parse_expression(lexer: &mut Lexer) -> Result<Expression, String> {
    expr_binding_power(lexer, 0)
}

fn expr_binding_power(
    lexer: &mut Lexer,
    min_binding_power: u8,
) -> Result<Expression, String> {
    let mut left = match lexer.next() {
        Some(Token::ValueString(v)) => Expression::String(v),
        Some(Token::ValueChar(v)) => Expression::Char(v),
        Some(Token::ValueInt(v)) => Expression::Int(v),
        Some(Token::ValueFloat(v)) => Expression::Float(v),
        Some(Token::ValueIdentifier(v)) => Expression::Identifier(v),

        Some(operator @ Token::Minus) => {
            let (_, right_binding_power) = prefix_binding_power(&operator)?;
            let right = expr_binding_power(lexer, right_binding_power)?;

            Expression::Prefix {
                operator,
                value: Box::new(right),
            }
        }

        Some(Token::LeftParenNormal) => {
            let inner_expression = parse_expression(lexer);

            if lexer.next() != Some(Token::RightParenNormal) {
                return Err(format!("Expected ("));
            }

            inner_expression?
        }

        Some(Token::LeftParenCurly) => parse_block(lexer)?,

        Some(Token::If) => parse_if_condition(lexer)?,

        None => return Err(format!("Expected expression")),
        Some(t) => return Err(format!("Unexpected token: {:?}", t)),
    };

    loop {
        let operator = match lexer.peek() {
            token @ Some(
                Token::Plus
                | Token::Minus
                | Token::Star
                | Token::Slash
                | Token::ExclamationMark,
            ) => token.unwrap(),

            _ => break,
        };

        match postfix_binding_power(&operator) {
            Ok((left_binding_power, ())) => {
                if left_binding_power < min_binding_power {
                    break;
                }

                lexer.next();

                left = Expression::Postfix {
                    value: Box::new(left),
                    operator,
                };
                continue;
            }

            _ => {}
        }

        match infix_binding_power(&operator) {
            Ok((left_binding_power, right_binding_power)) => {
                if left_binding_power < min_binding_power {
                    break;
                }

                lexer.next();
                let right = expr_binding_power(lexer, right_binding_power)?;

                left = Expression::Infix {
                    left: Box::new(left),
                    operator,
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

fn prefix_binding_power(op: &Token) -> Result<((), u8), String> {
    match op {
        Token::Minus => Ok(((), 30)),

        _ => Err(format!("Unexpected token: {:?}", op)),
    }
}

fn infix_binding_power(op: &Token) -> Result<(u8, u8), String> {
    match op {
        Token::Plus | Token::Minus => Ok((10, 11)),
        Token::Star | Token::Slash => Ok((20, 21)),

        _ => Err(format!("Unexpected token: {:?}", op)),
    }
}

fn postfix_binding_power(op: &Token) -> Result<(u8, ()), String> {
    match op {
        Token::ExclamationMark => Ok((40, ())),

        _ => Err(format!("Unexpected token: {:?}", op)),
    }
}
