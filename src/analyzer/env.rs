use crate::parser::r#type::Type;

#[derive(Debug, Clone)]
pub struct EnvironmentVariable {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug, Clone)]
pub struct EnvironmentFunction {
    pub name: String,
    pub param_types: Vec<Type>,
    pub return_type: Type,
}

#[derive(Debug, Clone)]
pub struct Environment<'a> {
    pub parent_env: Option<&'a Environment<'a>>,

    pub variables: Vec<EnvironmentVariable>,
    pub functions: Vec<EnvironmentFunction>,
    // TODO: add struct definitions, etc.
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            parent_env: None,
            variables: vec![],
            functions: vec![],
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

    pub fn get_function(&self, name: &str) -> Option<EnvironmentFunction> {
        for function in &self.functions {
            if function.name == name {
                return Some(function.clone());
            }
        }

        return None;
    }

    pub fn add_variable(&mut self, name: String, type_: Type) {
        self.variables.push(EnvironmentVariable { name, type_ })
    }

    pub fn add_function(
        &mut self,
        name: String,
        param_types: Vec<Type>,
        return_type: Type,
    ) {
        self.functions.push(EnvironmentFunction {
            name,
            param_types,
            return_type,
        })
    }
}
