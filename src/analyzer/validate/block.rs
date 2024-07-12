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
    let expressions_copy = expressions.clone();
    let len = expressions_copy.len();

    let expressions_without_last_expression = &expressions_copy[..len - 1];
    let last_expression = &expressions_copy[len - 1];

    for mut expression in expressions_without_last_expression {
        validate_and_get_type(&mut expression, &mut block_env)?;
    }

    return validate_and_get_type(last_expression, &mut block_env);
}
