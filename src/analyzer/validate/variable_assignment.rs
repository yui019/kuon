use crate::{
    analyzer::{analyzer_error::AnalyzerError, env::Environment},
    analyzer_error,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_variable_assignment(
    env: &mut Environment,
    line: usize,
    name: &String,
    value: &Expression,
) -> Result<Type, AnalyzerError> {
    let var = match env.get_variable(name) {
        None => {
            return analyzer_error!(
                line,
                "Variable with name {} does not exist",
                name
            )
        }
        Some(v) => v,
    };

    if var.constant {
        return analyzer_error!(
            line,
            "Cannot reassign constant variable {}",
            name
        );
    }

    let value_type = validate_and_get_type(value, env)?;

    if value_type != var.type_ {
        return analyzer_error!(
            value.line,
            "Expected value of type {:?}, got value of type {:?} instead",
            var.type_,
            value_type
        );
    }

    return Ok(Type::Null);
}
