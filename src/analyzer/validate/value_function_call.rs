use crate::{
    analyzer::{
        analyzer_error::AnalyzerError, env::Environment, util::types_equal,
    },
    analyzer_error,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_value_function_call(
    env: &mut Environment,
    pre_argument: &mut Expression,
    function_name: &String,
    arguments: &mut Vec<Expression>,
    pre_argument_type: &mut Option<Type>,
) -> Result<Type, AnalyzerError> {
    *pre_argument_type = Some(validate_and_get_type(pre_argument, env)?);

    let function = match env.get_function(&function_name, &pre_argument_type) {
        Some(function) => function,

        None => {
            return analyzer_error!(
                pre_argument.line,
                "Cannot find function {:?}:{}",
                pre_argument_type,
                function_name
            )
        }
    };

    if !types_equal(
        &env,
        pre_argument_type.as_ref().unwrap(),
        &function.pre_param_type.clone().unwrap(),
    ) {
        return analyzer_error!(
            pre_argument.line,
            "Expected pre-argument of type {:?}, got type {:?}",
            function.pre_param_type.unwrap(),
            pre_argument_type
        );
    }

    if arguments.len() != function.param_types.len() {
        return analyzer_error!(
            pre_argument.line,
            "Expected {} arguments, {} provided",
            function.param_types.len(),
            arguments.len()
        );
    }

    for i in 0..function.param_types.len() {
        let argument_type = validate_and_get_type(&mut arguments[i], env)?;
        let param_type = function.param_types[i].clone();

        if !types_equal(env, &argument_type, &param_type) {
            return analyzer_error!(
                arguments[i].line,
                "Expected value of type {:?}, got value of type {:?} instead",
                param_type,
                argument_type
            );
        }
    }

    return Ok(function.return_type);
}
