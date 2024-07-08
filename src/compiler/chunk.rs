use super::operation::Operation;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<Operation>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: vec![] }
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
