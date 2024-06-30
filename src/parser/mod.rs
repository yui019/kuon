use crate::lexer::{token::Token, Lexer};

#[derive(Debug, Clone)]
pub enum Expression {
    String(String),
    Char(char),
    Int(i64),
    Float(f64),
    Identifier(String),

    Prefix {
        operator: Token,
        value: Box<Expression>,
    },

    Infix {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },

    Postfix {
        value: Box<Expression>,
        operator: Token,
    },
}

pub fn parse(lexer: &mut Lexer) -> Result<Expression, String> {
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
            let left = expr_binding_power(lexer, 0);

            if lexer.next() != Some(Token::RightParenNormal) {
                return Err(format!("Expected ("));
            }

            left?
        }

        t => return Err(format!("Unexpected token: {:?}", t)),
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
