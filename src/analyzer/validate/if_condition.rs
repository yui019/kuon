use crate::{
    analyzer::{
        analyzer_error::AnalyzerError, env::Environment, util::types_equal,
    },
    analyzer_error,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_if_condition(
    env: &mut Environment,
    line: usize,
    condition: &mut Expression,
    true_branch: &mut Expression,
    else_branch: &mut Option<Box<Expression>>,
) -> Result<Type, AnalyzerError> {
    let condition_type = validate_and_get_type(condition, env)?;
    if !matches!(condition_type, Type::Bool) {
        return analyzer_error!(
            condition.line,
            "The condition needs to be a boolean"
        );
    }

    if else_branch.is_none() {
        todo!("Return a nullable type here");
    }

    let else_branch = else_branch.as_mut().unwrap();

    let true_type = validate_and_get_type(true_branch, env)?;
    let else_type = validate_and_get_type(else_branch, env)?;

    if !types_equal(env, &true_type, &else_type) {
        return analyzer_error!(
            line,
            "The true and else branch must have the same type"
        );
    }

    return Ok(true_type);
}
