use analyzer_error::AnalyzerError;
use env::Environment;

use crate::{
    analyzer::validate::validate_and_get_type, parser::expression::Expression,
};

mod analyzer_error;
mod env;
pub mod util;
mod validate;

pub fn validate(ast: &mut Expression) -> Result<(), AnalyzerError> {
    let mut root_env = Environment::new();

    validate_expression(ast, &mut root_env)?;

    return Ok(());
}

fn validate_expression(
    expression: &mut Expression,
    parent_env: &mut Environment,
) -> Result<(), AnalyzerError> {
    validate_and_get_type(expression, parent_env)?;

    return Ok(());
}
