use crate::compiler::value::Object;

#[derive(Debug)]
pub struct VmHeap {
    pub objects: Vec<Object>,
}

impl VmHeap {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    // Add an object and return its index
    pub fn add_object(&mut self, object: Object) -> usize {
        self.objects.push(object);

        return self.objects.len() - 1;
    }

    pub fn get_object(&mut self, index: usize) -> Object {
        self.objects[index].clone()
    }

    pub fn update_object(&mut self, index: usize, object: Object) {
        self.objects[index] = object;
    }
}
