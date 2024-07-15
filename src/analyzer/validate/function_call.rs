use crate::{
    analyzer::{analyzer_error::AnalyzerError, env::Environment},
    analyzer_error,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_function_call(
    env: &mut Environment,
    function: &Expression,
    arguments: &Vec<Expression>,
) -> Result<Type, AnalyzerError> {
    let function_type = validate_and_get_type(&function, env)?;

    let return_type: Type;
    let param_types: Vec<Type>;

    match function_type {
        Type::Function {
            return_type: a,
            param_types: b,
        } => {
            return_type = *a.clone();
            param_types = b;
        }

        _ => {
            return analyzer_error!(
                function.line,
                "Not a function: {:?}",
                *function
            )
        }
    };

    if arguments.len() != param_types.len() {
        return analyzer_error!(
            function.line,
            "Expected {} arguments, {} provided",
            param_types.len(),
            arguments.len()
        );
    }

    for i in 0..param_types.len() {
        let argument_type = validate_and_get_type(&arguments[i], env)?;
        let param_type = param_types[i].clone();

        if argument_type != param_type {
            return analyzer_error!(
                arguments[i].line,
                "Expected value of type {:?}, got value of type {:?} instead",
                param_type,
                argument_type
            );
        }
    }

    return Ok(return_type);
}
