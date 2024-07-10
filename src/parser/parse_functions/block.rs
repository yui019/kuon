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
        let first_token = lexer.peek();
        if first_token.is_none() {
            break;
        }

        let expression = parse_expression(lexer)?;

        if matches!(expression, Expression::FunctionDefinition { .. }) {
            return Err(parser_error!(
                first_token.unwrap().line,
                "Standalone function definitions aren't allowed below top-level"
            ));
        }

        expressions.push(expression.clone());

        // determine if the expression requires a semicolon after it

        let mut require_semicolon = true;

        match expression {
            // semicolons aren't required after if conditions
            Expression::IfCondition { .. } => {
                require_semicolon = false;
            }

            _ => {}
        }

        if require_semicolon {
            let next = lexer.peek();

            if !matches!(
                next,
                Some(token_data!(
                    TokenData::Semicolon | TokenData::RightParenCurly
                ))
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

        match lexer.peek() {
            Some(token_data!(TokenData::Semicolon)) => {
                lexer.next();

                if token_matches(&lexer.peek(), &TokenData::RightParenCurly) {
                    lexer.next();
                    expressions.push(Expression::Null);
                    break;
                }

                continue;
            }

            Some(token_data!(TokenData::RightParenCurly)) => {
                lexer.next();
                break;
            }

            Some(_) => continue,
            None => return Err(parser_error_eof!("Expected }}")),
        }
    }

    Ok(Expression::Block { expressions })
}
