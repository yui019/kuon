use crate::parser::r#type::Type;

#[derive(Debug, Clone)]
pub struct EnvironmentVariable {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug, Clone)]
pub struct Environment<'a> {
    pub parent_env: Option<&'a Environment<'a>>,

    pub variables: Vec<EnvironmentVariable>,
    // TODO: add functions, struct definitions, etc.
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            parent_env: None,
            variables: vec![],
        }
    }

    pub fn from_parent(parent_env: &'a Environment) -> Self {
        Self {
            parent_env: Some(parent_env),
            variables: vec![],
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

    pub fn add_variable(&mut self, name: String, type_: Type) {
        self.variables.push(EnvironmentVariable { name, type_ })
    }
}
