use crate::{
    analyzer::env::Environment,
    parser::{
        expression::{Expression, FunctionParam},
        r#type::Type,
    },
};

use super::validate_and_get_type;

pub fn validate_function_definition(
    env: &mut Environment,
    name: &Option<String>,
    params: &Vec<FunctionParam>,
    return_type: &Type,
    body: &Box<Expression>,
) -> Result<Type, String> {
    let mut param_types: Vec<Type> = vec![];
    for param in params {
        param_types.push(param.type_.clone());
    }

    // add function to the environment if it has a name (only top level
    // functions can have names, this is ensured by the parser)
    if name.is_some() {
        env.add_function(
            name.clone().unwrap(),
            param_types.clone(),
            return_type.clone(),
        );
    }

    // validate inner body of function
    let mut body_env = Environment::from_parent(&env);
    for param in params {
        body_env.add_variable(param.name.clone(), param.type_.clone());
    }
    let body_type = validate_and_get_type(&body, &mut body_env)?;

    if body_type != *return_type {
        return Err(format!(
            "Function should return {:?}, but it returns {:?}",
            return_type, body_type
        ));
    }

    return Ok(Type::Function {
        param_types,
        return_type: Box::new(return_type.clone()),
    });
}
