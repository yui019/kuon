use crate::{
    analyzer::{analyzer_error::AnalyzerError, env::Environment},
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_block(
    env: &mut Environment,
    expressions: &mut Vec<Expression>,
) -> Result<Type, AnalyzerError> {
    let mut block_env = Environment::from_parent(env);
    let len = expressions.len();

    let expressions_without_last_expression = &mut expressions[..len - 1];

    for mut expression in expressions_without_last_expression {
        validate_and_get_type(&mut expression, &mut block_env)?;
    }

    let last_expression = &mut expressions[len - 1];
    return validate_and_get_type(last_expression, &mut block_env);
}
