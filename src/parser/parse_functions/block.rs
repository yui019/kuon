use crate::{
    expression, expression_pat,
    lexer::{
        token::{Token, TokenData},
        Lexer,
    },
    parser::{expression::ExpressionData, parser_error::ParserError},
    parser_error, parser_error_eof, some_token_pat,
};

use super::super::{
    expression::Expression, parse_expression, util::token_matches,
};

/// Called after Token::LeftParenCurly
pub fn parse_block(
    lexer: &mut Lexer,
    line: usize,
) -> Result<Expression, ParserError> {
    let mut expressions: Vec<Expression> = vec![];

    if token_matches(&lexer.peek(), &TokenData::RightParenCurly) {
        let token = lexer.next().unwrap();
        expressions.push(expression!(Null, token.line));

        return Ok(expression!(Block { expressions }, line));
    }

    loop {
        let first_token = lexer.peek();
        if first_token.is_none() {
            break;
        }

        let expression = parse_expression(lexer)?;

        if matches!(
            expression,
            expression_pat!(ExpressionData::FunctionDefinition { .. })
        ) {
            return parser_error!(
                first_token.unwrap().line,
                "Standalone function definitions aren't allowed below top-level"
            );
        } else if matches!(
            expression,
            expression_pat!(ExpressionData::StructDefinition { .. })
        ) {
            return parser_error!(
                first_token.unwrap().line,
                "Standalone struct definitions aren't allowed below top-level"
            );
        }

        expressions.push(expression.clone());

        // determine if the expression requires a semicolon after it

        let mut require_semicolon = true;

        match expression {
            // semicolons aren't required after if conditions
            Expression {
                data: ExpressionData::IfCondition { .. },
                ..
            } => {
                require_semicolon = false;
            }

            _ => {}
        }

        if require_semicolon {
            let next = lexer.peek();

            if !matches!(
                next,
                some_token_pat!(
                    TokenData::Semicolon | TokenData::RightParenCurly
                )
            ) {
                if let Some(token) = next {
                    return parser_error!(
                        token.line,
                        "Expected semicolon, got {:?}",
                        token.data
                    );
                }
            }
        }

        match lexer.peek() {
            some_token_pat!(TokenData::Semicolon) => {
                lexer.next();

                if token_matches(&lexer.peek(), &TokenData::RightParenCurly) {
                    let token = lexer.next().unwrap();
                    expressions.push(expression!(Null, token.line));
                    break;
                }

                continue;
            }

            some_token_pat!(TokenData::RightParenCurly) => {
                lexer.next();
                break;
            }

            Some(_) => continue,
            None => return parser_error_eof!("Expected }}"),
        }
    }

    Ok(expression!(Block { expressions }, line))
}
