use std::collections::BTreeMap;

use crate::{
    analyzer::{
        analyzer_error::AnalyzerError, env::Environment, util::types_equal,
    },
    analyzer_error,
    parser::{
        expression::{Expression, VariableAccessor},
        r#type::Type,
    },
};

use super::validate_and_get_type;

fn extract_struct_fields(
    env: &mut Environment,
    line: usize,
    type_: &Type,
) -> Result<BTreeMap<String, Type>, AnalyzerError> {
    match type_ {
        Type::Struct { fields } => Ok(fields.clone()),

        Type::UserDefined(name) => match env.get_struct(name) {
            Some(s) => Ok(s.fields),

            None => analyzer_error!(line, "Struct {} does not exist", name),
        },

        _ => analyzer_error!(line, "Not a struct"),
    }
}

pub fn validate_variable_assignment(
    env: &mut Environment,
    line: usize,
    name: &String,
    accessors: &Vec<VariableAccessor>,
    value: &mut Expression,
) -> Result<Type, AnalyzerError> {
    let var = match env.get_variable(name) {
        None => {
            return analyzer_error!(
                line,
                "Variable with name {} does not exist",
                name
            )
        }
        Some(v) => v,
    };

    if var.constant {
        return analyzer_error!(
            line,
            "Cannot reassign constant variable {}",
            name
        );
    }

    // if there's no accessors just compare the variable and value types, else
    // check if each field exists and compare the last one's type with the value
    if accessors.is_empty() {
        let value_type = validate_and_get_type(value, env)?;

        if !types_equal(env, &value_type, &var.type_) {
            return analyzer_error!(
                value.line,
                "Expected value of type {:?}, got value of type {:?} instead",
                var.type_,
                value_type
            );
        }
    } else {
        let mut fields = extract_struct_fields(env, line, &var.type_)?;

        for (i, accessor) in accessors.iter().enumerate() {
            // there will also be array indexes later, so this is just some
            // setup in advance
            let field = match accessor {
                VariableAccessor::StructField(f) => f,
            };

            if fields.get(field).is_none() {
                // error
            }

            // if it's on the last field compare its type to the value type,
            // else prepare for the next iteration
            if i == accessors.len() - 1 {
                let field_type = &fields[field];
                let value_type = validate_and_get_type(value, env)?;

                if !types_equal(env, &value_type, field_type) {
                    return analyzer_error!(
                        value.line,
                        "Expected value of type {:?}, got value of type {:?} instead",
                        field_type, value_type
                    );
                }
            } else {
                fields = extract_struct_fields(env, line, &fields[field])?;
            }
        }
    }

    return Ok(Type::Null);
}
