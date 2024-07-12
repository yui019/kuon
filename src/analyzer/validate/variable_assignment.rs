use crate::{
    analyzer::env::Environment,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_variable_assignment(
    env: &mut Environment,
    name: &String,
    value: &Expression,
) -> Result<Type, String> {
    let var = match env.get_variable(name) {
        None => {
            return Err(format!("Variable with name {} does not exist", name))
        }
        Some(v) => v,
    };

    if var.constant {
        return Err(format!("Cannot reassign constant variable {}", name));
    }

    let value_type = validate_and_get_type(value, env)?;

    if value_type != var.type_ {
        return Err(format!(
            "Expected value of type {:?}, got value of type {:?} instead",
            var.type_, value_type
        ));
    }

    return Ok(Type::Null);
}
