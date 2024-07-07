use crate::{
    analyzer::env::Environment,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_block(
    env: &mut Environment,
    expressions: &Vec<Expression>,
) -> Result<Type, String> {
    let mut block_env = Environment::from_parent(env);
    let mut expressions_copy = expressions.clone();

    for mut expression in &mut expressions_copy {
        validate_and_get_type(&mut expression, &mut block_env)?;
    }

    let last_expression = &expressions_copy[expressions_copy.len() - 1];
    return validate_and_get_type(last_expression, &mut block_env);
}
