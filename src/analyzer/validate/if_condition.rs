use crate::{
    analyzer::env::Environment,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_if_condition(
    env: &mut Environment,
    condition: &Expression,
    true_branch: &Expression,
    else_branch: &Option<Box<Expression>>,
) -> Result<Type, String> {
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
