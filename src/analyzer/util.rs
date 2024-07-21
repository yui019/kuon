use crate::parser::r#type::Type;

use super::env::Environment;

pub fn types_equal(env: &Environment, type1: &Type, type2: &Type) -> bool {
    if type1 == type2 {
        return true;
    }

    // if one type is UserDefined and the other is Struct, convert the
    // UserDefined to Struct and then compare them
    match (type1, type2) {
        (Type::UserDefined(name1), Type::Struct { fields: fields2 }) => {
            if let Some(struct1) = env.get_struct(&name1) {
                let fields1 = struct1.fields;

                return fields1 == *fields2;
            }
        }
        (Type::Struct { fields: fields1 }, Type::UserDefined(name2)) => {
            if let Some(struct2) = env.get_struct(&name2) {
                let fields2 = struct2.fields;

                return *fields1 == fields2;
            }
        }

        _ => {}
    }

    false
}
