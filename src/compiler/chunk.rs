use std::collections::HashMap;

use crate::parser::r#type::Type;

use super::operation::Operation;

#[derive(Debug, Clone)]
pub struct ChunkFunctionParam {
    pub constant: bool,
}

#[derive(Debug, Clone)]
pub struct ChunkFunction {
    pub chunk: Chunk,
    pub pre_param: Option<ChunkFunctionParam>,
    pub parameters: Vec<ChunkFunctionParam>,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<Operation>,
    pub functions: Vec<ChunkFunction>,

    // this is used while compiling as a mapping between global function names
    // and their indices in the functions Vec field
    pub function_index_from_name: HashMap<String, usize>,

    // same thing as function_index_from_name except for value functions, so
    // it's also indexed by a type for the pre-parameter
    pub value_function_index_from_name: HashMap<(String, Type), usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            functions: vec![],
            function_index_from_name: HashMap::new(),
            value_function_index_from_name: HashMap::new(),
        }
    }

    pub fn add_operation(&mut self, operation: &Operation) {
        self.code.push(operation.clone());
    }

    pub fn update_operation(
        &mut self,
        address: usize,
        new_operation: &Operation,
    ) {
        self.code[address] = new_operation.clone();
    }

    pub fn get_latest_address(&self) -> usize {
        self.code.len() - 1
    }
}
