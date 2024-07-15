use crate::{
    analyzer::{analyzer_error::AnalyzerError, env::Environment},
    analyzer_error,
    parser::r#type::Type,
};

pub fn validate_identifier(
    env: &mut Environment,
    line: usize,
    identifier: &String,
) -> Result<Type, AnalyzerError> {
    if let Some(function) = env.get_function(&identifier) {
        return Ok(Type::Function {
            param_types: function.param_types,
            return_type: Box::new(function.return_type),
        });
    } else if let Some(variable) = env.get_variable(&identifier) {
        return Ok(variable.type_);
    } else {
        return analyzer_error!(line, "Unknown variable: {}", identifier);
    }
}
