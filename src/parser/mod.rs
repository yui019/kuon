use expression::Expression;
use parse_functions::if_condition::parse_if_condition;
use parse_functions::variable_definition::parse_variable_definition;

use crate::lexer::{token::Token, Lexer};
use crate::parser::parse_functions::block::parse_block;

pub mod expression;
mod parse_functions;
pub mod r#type;

pub fn parse_source(lexer: &mut Lexer) -> Result<Expression, String> {
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
                return Err(format!("Expected semicolon, got {:?}", token))
            }

            None => break,
        }
    }

    Ok(Expression::Block { expressions })
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
            let (_, right_binding_power) =
                prefix_binding_power(&operator).unwrap();
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
        token @ Some(Token::Val | Token::Var) => {
            parse_variable_definition(lexer, token.unwrap())?
        }

        None => return Err(format!("Expected expression")),
        Some(t) => return Err(format!("Unexpected token: {:?}", t)),
    };

    loop {
        let operator = match lexer.peek() {
            Some(token) => token,

            _ => break,
        };

        match postfix_binding_power(&operator) {
            Some((left_binding_power, ())) => {
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
            Some((left_binding_power, right_binding_power)) => {
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

fn prefix_binding_power(op: &Token) -> Option<((), u8)> {
    match op {
        Token::Minus => Some(((), 30)),

        _ => None,
    }
}

fn infix_binding_power(op: &Token) -> Option<(u8, u8)> {
    match op {
        Token::Plus | Token::Minus => Some((10, 11)),
        Token::Star | Token::Slash => Some((20, 21)),

        Token::EqualsEquals
        | Token::LessThan
        | Token::LessThanOrEqual
        | Token::GreaterThan
        | Token::GreaterThanOrEqual => Some((0, 1)),

        _ => None,
    }
}

fn postfix_binding_power(op: &Token) -> Option<(u8, ())> {
    match op {
        // Token::ExclamationMark => Some((40, ())),
        _ => None,
    }
}
