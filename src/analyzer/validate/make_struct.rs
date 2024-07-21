use std::collections::HashMap;

use crate::{
    analyzer::{
        analyzer_error::AnalyzerError, env::Environment, util::types_equal,
    },
    analyzer_error,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_make_struct(
    env: &mut Environment,
    line: usize,
    name: &Option<String>,
    fields: &HashMap<String, Expression>,
) -> Result<Type, AnalyzerError> {
    // check if fields match the predefined struct if a name is given
    if name.is_some() {
        let name = name.clone().unwrap();

        let defined_fields = match env.get_struct(&name) {
            Some(s) => s.fields,
            None => {
                return analyzer_error!(
                    line,
                    "No struct with the name {} found",
                    name
                )
            }
        };

        if defined_fields.len() != fields.len() {
            return analyzer_error!(
                line,
                "Expected {} fields, {} given",
                defined_fields.len(),
                fields.len()
            );
        }

        for (defined_field_name, defined_field_type) in defined_fields {
            match fields.get(&defined_field_name) {
                Some(field_value) => {
                    let field_type = validate_and_get_type(field_value, env)?;

                    if !types_equal(env, &field_type, &defined_field_type) {
                        return analyzer_error!(
                            line,
                            "Field {} should have value of type {:?}, got value of type {:?}",
                            defined_field_name,
                            defined_field_type,
                            field_type
                        );
                    }
                }

                None => {
                    return analyzer_error!(
                        line,
                        "Field {} is missing",
                        defined_field_name
                    )
                }
            }
        }

        return Ok(Type::UserDefined(name));
    } else {
        // if there's no name, just validate all the fields
        let mut field_types: HashMap<String, Type> = HashMap::new();
        for (field_name, field_value) in fields {
            // duplicate fields not allowed
            if field_types.contains_key(field_name) {
                return analyzer_error!(
                    line,
                    "Duplicate field: {}",
                    field_name
                );
            }

            field_types.insert(
                field_name.clone(),
                validate_and_get_type(field_value, env)?,
            );
        }

        return Ok(Type::Struct {
            fields: field_types,
        });
    }
}
