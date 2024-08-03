use std::collections::BTreeMap;

use crate::{
    analyzer::{analyzer_error::AnalyzerError, env::Environment},
    analyzer_error,
    parser::r#type::Type,
};

pub fn validate_struct_definition(
    env: &mut Environment,
    line: usize,
    name: &Option<String>,
    fields: &BTreeMap<String, Type>,
) -> Result<Type, AnalyzerError> {
    // add struct to the environment if it has a name (only top level structs
    // can have names, this is ensured by the parser)
    if name.is_some() {
        let name = name.clone().unwrap();

        if env.get_struct(&name).is_some() {
            return analyzer_error!(
                line,
                "A struct with the name {} already exists",
                name
            );
        }

        env.add_struct(name, fields.clone());
    }

    return Ok(Type::Null);
}
