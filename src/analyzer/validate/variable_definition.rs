use crate::{
    analyzer::env::Environment,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_variable_definition(
    env: &mut Environment,
    type_: &Option<Box<Type>>,
    value: &Expression,
    name: &String,
) -> Result<Type, String> {
    if env.get_variable(name).is_some() {
        return Err(format!("Variable with name {} already exists", name));
    }

    if let Some(type_) = type_ {
        if **type_ != validate_and_get_type(&value, env)? {
            return Err(format!(
                "Cannot cast value {:?} to type {:?}",
                value, type_
            ));
        }

        env.add_variable(name.clone(), *type_.clone());
    } else {
        let type_ = validate_and_get_type(&value, env)?;
        env.add_variable(name.clone(), type_);
    }

    return Ok(Type::Null);
}
