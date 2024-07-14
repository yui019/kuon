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

    // Index of this function once it's been added to the chunk (calculated in
    // advance)
    let index = chunk.functions.len();

    if name.is_some() {
        let name = name.clone().unwrap();

        // add mapping from this function name to the precalculated index
        chunk.function_index_from_name.insert(name.clone(), index);

        // make the same mapping in the function's chunk (this index doesn't
        // actually exist in the function's chunk, but it will be executed from
        // the parent chunk so this makes it easier for the vm - the alternative
        // would be making some sort of special operation for a function to call
        // itself)
        function_chunk.function_index_from_name.insert(name, index);
    }

    // compile function body
    compile_expression(&mut function_chunk, body, is_function)?;
    function_chunk.add_operation(&Operation::Halt);

    // add function to the chunk
    chunk.functions.push(ChunkFunction {
        chunk: function_chunk,
        parameter_count: params.len() as u32,
    });

    Ok(index)
}
