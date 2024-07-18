use crate::{
    analyzer::{analyzer_error::AnalyzerError, env::Environment},
    analyzer_error,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_variable_definition(
    env: &mut Environment,
    line: usize,
    type_: &Option<Box<Type>>,
    value: &Expression,
    name: &String,
    constant: bool,
) -> Result<Type, AnalyzerError> {
    if env.get_variable(name).is_some() {
        return analyzer_error!(
            line,
            "Variable with name {} already exists",
            name
        );
    }

    if env.get_function(name).is_some() {
        return analyzer_error!(
            line,
            "Name {} is taken by an existing function",
            name
        );
    }

    if let Some(type_) = type_ {
        if **type_ != validate_and_get_type(&value, env)? {
            return analyzer_error!(
                value.line,
                "Cannot cast value {:?} to type {:?}",
                value,
                type_
            );
        }

        env.add_variable(name.clone(), *type_.clone(), constant);
    } else {
        let type_ = validate_and_get_type(&value, env)?;
        env.add_variable(name.clone(), type_, constant);
    }

    return Ok(Type::Null);
}
