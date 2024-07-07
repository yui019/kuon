use crate::{analyzer::env::Environment, parser::r#type::Type};

pub fn validate_identifier(
    env: &mut Environment,
    identifier: &String,
) -> Result<Type, String> {
    if let Some(function) = env.get_function(&identifier) {
        return Ok(Type::Function {
            param_types: function.param_types,
            return_type: Box::new(function.return_type),
        });
    } else if let Some(variable) = env.get_variable(&identifier) {
        return Ok(variable.type_);
    } else {
        return Err(format!("Unknown variable: {}", identifier));
    }
}
