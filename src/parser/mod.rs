use expression::{Expression, ExpressionData};
use parse_functions::create_variable_assignment::create_variable_assignment;
use parse_functions::function_arguments::parse_function_arguments;
use parse_functions::function_definition::parse_function_definition;
use parse_functions::if_condition::parse_if_condition;
use parse_functions::make_struct::parse_make_struct;
use parse_functions::struct_definition::parse_struct_definition;
use parse_functions::variable_definition::parse_variable_definition;
use parser_error::ParserError;
use util::token_matches;

use crate::lexer::token::TokenData;
use crate::lexer::{token::Token, Lexer};
use crate::parser::parse_functions::block::parse_block;
use crate::{expression, some_token_pat, token_pat};

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
            let expr = parse_expression_top_level(lexer)?;
            expressions.push(expr.clone());

            // determine if the expression requires a semicolon after it

            let mut require_semicolon = true;

            match expr {
                Expression {
                    data: ExpressionData::FunctionDefinition { name, .. },
                    ..
                } => {
                    if name.is_none() {
                        return parser_error!(
                            token.unwrap().line,
                            "Top level function definitions require a name"
                        );
                    }

                    require_semicolon = false;
                }

                Expression {
                    data: ExpressionData::StructDefinition { name, .. },
                    ..
                } => {
                    if name.is_none() {
                        return parser_error!(
                            token.unwrap().line,
                            "Top level struct definitions require a name"
                        );
                    }

                    require_semicolon = false;
                }

                Expression {
                    data: ExpressionData::IfCondition { .. },
                    ..
                } => {
                    require_semicolon = false;
                }

                _ => {}
            }

            if require_semicolon {
                let next = lexer.next();

                if !matches!(next, some_token_pat!(TokenData::Semicolon)) {
                    if let Some(token) = next {
                        return parser_error!(
                            token.line,
                            "Expected semicolon, got {:?}",
                            token.data
                        );
                    }
                }
            }
        } else {
            break;
        }
    }

    Ok(expression!(Block { expressions }, 0))
}

fn parse_expression_top_level(
    lexer: &mut Lexer,
) -> Result<Expression, ParserError> {
    expr_binding_power(lexer, 0, true)
}

fn parse_expression(lexer: &mut Lexer) -> Result<Expression, ParserError> {
    expr_binding_power(lexer, 0, false)
}

fn expr_binding_power(
    lexer: &mut Lexer,
    min_binding_power: u8,
    top_level: bool,
) -> Result<Expression, ParserError> {
    use TokenData::*;

    let mut left = match lexer.next() {
        some_token_pat!(ValueString(v), line) => {
            expression!(String(v), line)
        }
        some_token_pat!(ValueChar(v), line) => {
            expression!(Char(v), line)
        }
        some_token_pat!(ValueInt(v), line) => {
            expression!(Int(v), line)
        }
        some_token_pat!(ValueFloat(v), line) => {
            expression!(Float(v), line)
        }
        some_token_pat!(True, line) => {
            expression!(Bool(true), line)
        }
        some_token_pat!(False, line) => {
            expression!(Bool(false), line)
        }
        some_token_pat!(ValueIdentifier(v), line) => {
            if matches!(lexer.peek(), some_token_pat!(LeftParenCurly))
                && min_binding_power == 0
            {
                parse_make_struct(lexer, line, Some(v))?
            } else {
                expression!(Identifier(v), line)
            }
        }

        Some(operator @ token_pat!(TokenData::Minus, line)) => {
            let (_, right_binding_power) =
                prefix_binding_power(&operator.data).unwrap();
            let right = expr_binding_power(lexer, right_binding_power, false)?;

            expression!(
                Prefix {
                    operator: operator.data,
                    value: Box::new(right),
                },
                line
            )
        }

        some_token_pat!(LeftParenNormal) => {
            let inner_expression = parse_expression(lexer);

            let next = lexer.next();
            if !token_matches(&next, &TokenData::RightParenNormal) {
                return parser_error!(next.unwrap().line, "Expected (");
            }

            inner_expression?
        }

        some_token_pat!(TokenData::LeftParenCurly, line) => {
            parse_block(lexer, line)?
        }

        some_token_pat!(TokenData::If, line) => {
            parse_if_condition(lexer, line)?
        }

        token @ some_token_pat!(TokenData::Val | TokenData::Var) => {
            parse_variable_definition(lexer, token.unwrap())?
        }

        some_token_pat!(TokenData::Fun, line) => {
            parse_function_definition(lexer, top_level, line)?
        }

        some_token_pat!(TokenData::Struct, line) => {
            parse_struct_definition(lexer, top_level, line)?
        }

        some_token_pat!(TokenData::MkStruct, line) => {
            parse_make_struct(lexer, line, None)?
        }

        None => return parser_error_eof!("Expected expression"),
        Some(t) => {
            return parser_error!(t.line, "Unexpected token: {:?}", t.data)
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
                    TokenData::LeftParenNormal => expression!(
                        FunctionCall {
                            function: Box::new(left.clone()),
                            arguments: parse_function_arguments(lexer)?,
                        },
                        left.line
                    ),

                    // else
                    _ => expression!(
                        Postfix {
                            value: Box::new(left.clone()),
                            operator: operator.data,
                        },
                        left.line
                    ),
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
                let right =
                    expr_binding_power(lexer, right_binding_power, false)?;

                left = match operator.data {
                    // variable assignment
                    TokenData::Equals => {
                        create_variable_assignment(&left, &right)?
                    }

                    // Field access
                    TokenData::Dot => {
                        let field = match right {
                            Expression {
                                data: ExpressionData::Identifier(identifier),
                                ..
                            } => identifier,

                            _ => {
                                return parser_error!(
                                    right.line,
                                    "Field should be an identifier"
                                );
                            }
                        };

                        expression!(
                            FieldAccess {
                                expression: Box::new(left.clone()),
                                field
                            },
                            left.line
                        )
                    }

                    // else
                    _ => expression!(
                        Infix {
                            left: Box::new(left.clone()),
                            operator: operator.data,
                            right: Box::new(right),
                        },
                        left.line
                    ),
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
        TokenData::Minus => Some(((), 40)),

        _ => None,
    }
}

fn infix_binding_power(op: &TokenData) -> Option<(u8, u8)> {
    match op {
        TokenData::Plus | TokenData::Minus => Some((20, 21)),
        TokenData::Star | TokenData::Slash => Some((30, 31)),

        TokenData::EqualsEquals
        | TokenData::LessThan
        | TokenData::LessThanOrEqual
        | TokenData::GreaterThan
        | TokenData::GreaterThanOrEqual => Some((10, 11)),

        TokenData::Equals => Some((0, 0)),

        TokenData::Dot => Some((50, 51)),

        _ => None,
    }
}

fn postfix_binding_power(op: &TokenData) -> Option<(u8, ())> {
    match op {
        TokenData::LeftParenNormal => Some((50, ())),
        _ => None,
    }
}
