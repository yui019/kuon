use env::Environment;

use crate::{
    analyzer::validate_and_get_type::validate_and_get_type,
    parser::expression::Expression,
};

mod env;
mod validate_and_get_type;

pub fn validate(ast: &Vec<Expression>) -> Result<(), String> {
    let mut root_env = Environment::new();

    for expression in ast {
        validate_expression(expression, &mut root_env)?;
    }

    return Ok(());
}

fn validate_expression(
    expression: &Expression,
    parent_env: &mut Environment,
) -> Result<(), String> {
    validate_and_get_type(expression, parent_env)?;

    return Ok(());
}
