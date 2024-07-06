use crate::{
    lexer::token::TokenData,
    parser::{expression::Expression, r#type::Type},
};

use super::env::Environment;

pub fn validate_and_get_type(
    expression: &Expression,
    env: &mut Environment,
) -> Result<Type, String> {
    match expression {
        Expression::Null => return Ok(Type::Null),

        Expression::String(_) => return Ok(Type::String),

        Expression::Char(_) => return Ok(Type::Char),

        Expression::Int(_) => return Ok(Type::Int),

        Expression::Float(_) => return Ok(Type::Float),

        Expression::Identifier(identifier) => {
            if let Some(variable) = env.get_variable(&identifier) {
                return Ok(variable.type_);
            } else {
                return Err(format!("Unknown variable: {}", identifier));
            }
        }

        Expression::Prefix { operator, value } => {
            if *operator != TokenData::Minus {
                unreachable!();
            }

            match validate_and_get_type(value, env)? {
                type_ @ (Type::Int | Type::Uint)  => return Ok(type_),

                type_ => return Err(format!("Prefix operator - can not work on an expression of type {:?}", type_)),
            }
        }

        Expression::Infix {
            left,
            operator,
            right,
        } => {
            if *operator == TokenData::Minus
                || *operator == TokenData::Plus
                || *operator == TokenData::Star
                || *operator == TokenData::Slash
            {
                let left_type = validate_and_get_type(left, env)?;
                let right_type = validate_and_get_type(right, env)?;

                match (left_type, right_type) {
                    (Type::Uint, Type::Uint)
                        if *operator != TokenData::Minus =>
                    {
                        return Ok(Type::Uint);
                    }

                    (
                        Type::Int | Type::Uint | Type::Float,
                        Type::Int | Type::Uint | Type::Float,
                    ) => {
                        return Ok(Type::Int);
                    }

                    _ => {
                        return Err(format!(
                            "Operator {:?} only works on numbers",
                            operator
                        ))
                    }
                }
            } else if *operator == TokenData::LessThan
                || *operator == TokenData::LessThanOrEqual
                || *operator == TokenData::GreaterThan
                || *operator == TokenData::GreaterThanOrEqual
            {
                let left_type = validate_and_get_type(left, env)?;
                let right_type = validate_and_get_type(right, env)?;

                match (left_type, right_type) {
                    (
                        Type::Int | Type::Uint | Type::Float,
                        Type::Int | Type::Uint | Type::Float,
                    ) => {
                        return Ok(Type::Bool);
                    }

                    _ => {
                        return Err(format!(
                            "Operator {:?} only works on numbers",
                            operator
                        ))
                    }
                }
            } else if *operator == TokenData::EqualsEquals {
                let left_type = validate_and_get_type(left, env)?;
                let right_type = validate_and_get_type(right, env)?;

                if left_type == right_type {
                    return Ok(Type::Bool);
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
            if !matches!(condition_type, Type::Bool) {
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

            return Ok(Type::Null);
        }

        Expression::FunctionDefinition {
            params,
            return_type,
            ..
        } => {
            let mut param_types: Vec<Type> = vec![];
            for param in params {
                param_types.push(param.type_.clone());
            }

            return Ok(Type::Function {
                param_types,
                return_type: Box::new(return_type.clone()),
            });
        }

        Expression::Type { .. } => {
            return Err(format!("Cannot use a type as an expression"))
        }
    }
}
