use crate::{
    analyzer::env::Environment,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_function_call(
    env: &mut Environment,
    function: &Expression,
    arguments: &Vec<Expression>,
) -> Result<Type, String> {
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

        _ => return Err(format!("Not a function: {:?}", *function)),
    };

    if arguments.len() != param_types.len() {
        return Err(format!(
            "Expected {} arguments, {} provided",
            param_types.len(),
            arguments.len()
        ));
    }

    for i in 0..param_types.len() {
        let argument_type = validate_and_get_type(&arguments[i], env)?;
        let param_type = param_types[i].clone();

        if argument_type != param_type {
            return Err(format!(
                "Expected value of type {:?}, got value of type {:?} instead",
                param_type, argument_type
            ));
        }
    }

    return Ok(return_type);
}
