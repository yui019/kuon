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
}
