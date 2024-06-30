use crate::lexer::{token::Token, Lexer};

use super::super::{expression::Expression, parse_expression};

/// Called after Token::LeftParenCurly
pub fn parse_block(lexer: &mut Lexer) -> Result<Expression, String> {
    let mut expressions: Vec<Expression> = vec![];

    if lexer.peek() == Some(Token::RightParenCurly) {
        lexer.next();
        expressions.push(Expression::Null);

        return Ok(Expression::Block { expressions });
    }

    loop {
        expressions.push(parse_expression(lexer)?);

        match lexer.next() {
            Some(Token::Semicolon) => {
                if lexer.peek() == Some(Token::RightParenCurly) {
                    lexer.next();
                    expressions.push(Expression::Null);
                    break;
                }

                continue;
            }
            Some(Token::RightParenCurly) => break,

            Some(t) => {
                return Err(format!(
                    "Expected semicolon inside block, got {:?}",
                    t
                ))
            }
            None => return Err(format!("Expected }}")),
        }
    }

    Ok(Expression::Block { expressions })
}
