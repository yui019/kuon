use crate::{
    analyzer::{
        analyzer_error::AnalyzerError, env::Environment, util::types_equal,
    },
    analyzer_error,
    parser::{
        expression::{Expression, FunctionParam},
        r#type::Type,
    },
};

use super::validate_and_get_type;

pub fn validate_function_definition(
    env: &mut Environment,
    line: usize,
    name: &Option<String>,
    params: &Vec<FunctionParam>,
    return_type: &Type,
    body: &Box<Expression>,
) -> Result<Type, AnalyzerError> {
    let mut param_types: Vec<Type> = vec![];
    for param in params {
        param_types.push(param.type_.clone());
    }

    // add function to the environment if it has a name (only top level
    // functions can have names, this is ensured by the parser)
    if name.is_some() {
        let name = name.clone().unwrap();

        if env.get_function(&name).is_some() {
            return analyzer_error!(
                line,
                "A function with the name {} already exists",
                name
            );
        }

        env.add_function(name, param_types.clone(), return_type.clone());
    }

    // validate inner body of function
    let mut body_env = Environment::from_parent(&env);

    // add function params to its environment before validating its body
    for param in params {
        let constant = true;

        body_env.add_variable(
            param.name.clone(),
            param.type_.clone(),
            constant,
        );
    }
    let body_type = validate_and_get_type(&body, &mut body_env)?;

    if !types_equal(env, &body_type, return_type) {
        return analyzer_error!(
            body.line,
            "Function should return {:?}, but it returns {:?}",
            return_type,
            body_type
        );
    }

    return Ok(Type::Function {
        param_types,
        return_type: Box::new(return_type.clone()),
    });
}
