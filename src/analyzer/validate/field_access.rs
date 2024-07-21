use crate::{
    analyzer::{analyzer_error::AnalyzerError, env::Environment},
    analyzer_error,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_field_access(
    env: &mut Environment,
    expression: &Expression,
    field: &String,
) -> Result<Type, AnalyzerError> {
    let expression_type = validate_and_get_type(expression, env)?;

    let fields = match expression_type {
        Type::Struct { fields } => fields,

        Type::UserDefined(name) => match env.get_struct(&name) {
            Some(s) => s.fields,
            None => {
                return analyzer_error!(
                expression.line,
                "Expression {:?} is not a struct, cannot access a field on it",
                expression
            )
            }
        },

        _ => {
            return analyzer_error!(
                expression.line,
                "Expression {:?} is not a struct, cannot access a field on it",
                expression
            )
        }
    };

    match fields.get(field) {
        Some(type_) => Ok(type_.clone()),
        None => analyzer_error!(
            expression.line,
            "Field {:?} does not exist on expression {:?}",
            field,
            expression
        ),
    }
}
