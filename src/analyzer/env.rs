use std::collections::BTreeMap;

use crate::parser::r#type::Type;

use super::util::types_equal;

#[derive(Debug, Clone)]
pub struct EnvironmentVariable {
    pub name: String,
    pub type_: Type,
    pub constant: bool,
}

#[derive(Debug, Clone)]
pub struct EnvironmentFunction {
    pub name: String,
    pub pre_param_type: Option<Type>,
    pub param_types: Vec<Type>,
    pub return_type: Type,
}

#[derive(Debug, Clone)]
pub struct EnvironmentStruct {
    pub name: String,
    pub fields: BTreeMap<String, Type>,
}

#[derive(Debug, Clone)]
pub struct Environment<'a> {
    pub parent_env: Option<&'a Environment<'a>>,

    pub variables: Vec<EnvironmentVariable>,
    pub functions: Vec<EnvironmentFunction>,
    pub structs: Vec<EnvironmentStruct>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            parent_env: None,
            variables: vec![],
            functions: vec![],
            structs: vec![],
        }
    }

    pub fn from_parent(parent_env: &'a Environment) -> Self {
        Self {
            parent_env: Some(parent_env),
            variables: vec![],

            // unlike variables, this field is only there for top-level
            // functions so it makes sense to copy them down to each
            // environment because otherwise you would need to go up
            // and up to the top level one for every function call
            functions: parent_env.functions.clone(),

            // same goes for structs
            structs: parent_env.structs.clone(),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<EnvironmentVariable> {
        for variable in &self.variables {
            if variable.name == name {
                return Some(variable.clone());
            }
        }

        match self.parent_env {
            Some(parent_env) => parent_env.get_variable(name),
            None => None,
        }
    }

    pub fn get_function(
        &self,
        name: &str,
        pre_param_type: &Option<Type>,
    ) -> Option<EnvironmentFunction> {
        if pre_param_type.is_none() {
            for function in &self.functions {
                if function.name == name {
                    if function.pre_param_type.is_none() {
                        return Some(function.clone());
                    }
                }
            }
        } else {
            for function in &self.functions {
                if function.name == name {
                    if let Some(found_function_pre_param_type) =
                        &function.pre_param_type
                    {
                        if types_equal(
                            self,
                            pre_param_type.as_ref().unwrap(),
                            &found_function_pre_param_type,
                        ) {
                            return Some(function.clone());
                        }
                    }
                }
            }
        }

        return None;
    }

    pub fn get_struct(&self, name: &str) -> Option<EnvironmentStruct> {
        for struct_ in &self.structs {
            if struct_.name == name {
                return Some(struct_.clone());
            }
        }

        return None;
    }

    pub fn add_variable(&mut self, name: String, type_: Type, constant: bool) {
        self.variables.push(EnvironmentVariable {
            name,
            type_,
            constant,
        })
    }

    pub fn add_function(
        &mut self,
        name: String,
        pre_param_type: Option<Type>,
        param_types: Vec<Type>,
        return_type: Type,
    ) {
        self.functions.push(EnvironmentFunction {
            name,
            pre_param_type,
            param_types,
            return_type,
        })
    }

    pub fn add_struct(&mut self, name: String, fields: BTreeMap<String, Type>) {
        self.structs.push(EnvironmentStruct { name, fields })
    }
}
