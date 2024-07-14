use crate::{
    compiler::{
        chunk::{Chunk, ChunkFunction},
        compile_expression,
        operation::Operation,
    },
    parser::expression::{Expression, FunctionParam},
};

/// Returns function index if successful
pub fn compile_function_definition(
    chunk: &mut Chunk,
    is_function: bool,
    params: &Vec<FunctionParam>,
    body: &Expression,
    name: &Option<String>,
) -> Result<usize, String> {
    let mut function_chunk = Chunk::new();

    // store all params from the stack into variables
    for param in params {
        function_chunk.add_operation(&Operation::Store(param.name.clone()))
    }

    // compile function body
    compile_expression(&mut function_chunk, body, is_function)?;
    function_chunk.add_operation(&Operation::Halt);

    let function = ChunkFunction {
        chunk: function_chunk,
        parameter_count: params.len() as u32,
    };

    chunk.functions.push(function);
    let index = chunk.functions.len() - 1;

    if name.is_some() {
        let name = name.clone().unwrap();

        chunk.function_index_from_name.insert(name, index);
    }

    Ok(index)
}
