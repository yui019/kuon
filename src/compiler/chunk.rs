use std::collections::HashMap;

use super::operation::Operation;

#[derive(Debug, Clone)]
pub struct ChunkFunction {
    pub chunk: Chunk,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<Operation>,
    pub functions: Vec<ChunkFunction>,

    // this is used while compiling as a mapping between global function names
    // and their indices in the functions Vec field
    pub function_index_from_name: HashMap<String, usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            functions: vec![],
            function_index_from_name: HashMap::new(),
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
