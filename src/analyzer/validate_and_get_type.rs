use crate::{
    lexer::token::Token,
    parser::{expression::Expression, r#type::Type},
};

use super::env::Environment;

pub fn validate_and_get_type(
    expression: &Expression,
    env: &mut Environment,
) -> Result<Type, String> {
    match expression {
        Expression::Null => return Ok(Type { name: Token::Null }),

        Expression::String(_) => {
            return Ok(Type {
                name: Token::String,
            })
        }

        Expression::Char(_) => return Ok(Type { name: Token::Char }),

        Expression::Int(_) => return Ok(Type { name: Token::Int }),

        Expression::Float(_) => return Ok(Type { name: Token::Float }),

        Expression::Identifier(identifier) => {
            if let Some(variable) = env.get_variable(&identifier) {
                return Ok(variable.type_);
            } else {
                return Err(format!("Unknown variable: {}", identifier));
            }
        }

        Expression::Prefix { operator, value } => {
            if *operator != Token::Minus {
                unreachable!();
            }

            match validate_and_get_type(value, env)? {
                type_ @ Type {
                    name: Token::Int | Token::Uint,
                } => return Ok(type_),

                type_ => return Err(format!("Prefix operator - can not work on an expression of type {:?}", type_)),
            }
        }

        Expression::Infix {
            left,
            operator,
            right,
        } => {
            if *operator == Token::Minus
                || *operator == Token::Plus
                || *operator == Token::Star
                || *operator == Token::Slash
            {
                let left_type = validate_and_get_type(left, env)?;
                let right_type = validate_and_get_type(right, env)?;

                match (left_type, right_type) {
                    (
                        Type { name: Token::Uint },
                        Type { name: Token::Uint },
                    ) if *operator != Token::Minus => {
                        return Ok(Type { name: Token::Uint });
                    }

                    (
                        Type {
                            name: Token::Int | Token::Uint | Token::Float,
                        },
                        Type {
                            name: Token::Int | Token::Uint | Token::Float,
                        },
                    ) => {
                        return Ok(Type { name: Token::Int });
                    }

                    _ => {
                        return Err(format!(
                            "Operator {:?} only works on numbers",
                            operator
                        ))
                    }
                }
            } else if *operator == Token::LessThan
                || *operator == Token::LessThanOrEqual
                || *operator == Token::GreaterThan
                || *operator == Token::GreaterThanOrEqual
            {
                let left_type = validate_and_get_type(left, env)?;
                let right_type = validate_and_get_type(right, env)?;

                match (left_type, right_type) {
                    (
                        Type {
                            name: Token::Int | Token::Uint | Token::Float,
                        },
                        Type {
                            name: Token::Int | Token::Uint | Token::Float,
                        },
                    ) => {
                        return Ok(Type { name: Token::Bool });
                    }

                    _ => {
                        return Err(format!(
                            "Operator {:?} only works on numbers",
                            operator
                        ))
                    }
                }
            } else if *operator == Token::EqualsEquals {
                let left_type = validate_and_get_type(left, env)?;
                let right_type = validate_and_get_type(right, env)?;

                if left_type == right_type {
                    return Ok(Type { name: Token::Bool });
                } else {
                    return Err(format!(
                        "Operator {:?} only works on operands of the same type",
                        operator
                    ));
                }
            } else {
                unreachable!();
            }
        }

        Expression::Postfix { .. } => todo!(),

        Expression::Block { expressions } => {
            let mut block_env = Environment::from_parent(env);
            let mut expressions_copy = expressions.clone();

            for mut expression in &mut expressions_copy {
                validate_and_get_type(&mut expression, &mut block_env)?;
            }

            let last_expression = &expressions_copy[expressions_copy.len() - 1];
            return validate_and_get_type(last_expression, &mut block_env);
        }

        Expression::IfCondition {
            condition,
            true_branch,
            else_branch,
        } => {
            let condition_type = validate_and_get_type(&condition, env)?;
            if !matches!(condition_type, Type { name: Token::Bool }) {
                return Err(format!("The condition needs to be a boolean"));
            }

            if else_branch.is_none() {
                todo!("Return a nullable type here");
            }

            let else_branch = else_branch.clone().unwrap();

            let true_type = validate_and_get_type(&true_branch, env)?;
            let else_type = validate_and_get_type(&else_branch, env)?;

            if true_type != else_type {
                return Err(format!(
                    "The true and else branch must have the same type"
                ));
            }

            return Ok(true_type);
        }

        Expression::VariableDefinition {
            type_, value, name, ..
        } => {
            if let Some(type_) = type_ {
                if **type_ != validate_and_get_type(&value, env)? {
                    return Err(format!(
                        "Cannot cast value {:?} to type {:?}",
                        value, type_
                    ));
                }

                env.add_variable(name.clone(), *type_.clone());
            } else {
                let type_ = validate_and_get_type(&value, env)?;
                env.add_variable(name.clone(), type_);
            }

            return Ok(Type { name: Token::Null });
        }

        Expression::Type { .. } => {
            return Err(format!("Cannot use a type as an expression"))
        }
    }
}
