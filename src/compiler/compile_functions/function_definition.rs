use crate::{
    compiler::{
        chunk::{Chunk, ChunkFunction, ChunkFunctionParam},
        compile_expression,
        operation::Operation,
    },
    parser::expression::{Expression, FunctionParam},
};

/// Returns function index if successful
pub fn compile_function_definition(
    chunk: &mut Chunk,
    is_function: bool,
    pre_param: &Option<FunctionParam>,
    params: &Vec<FunctionParam>,
    body: &Expression,
    name: &Option<String>,
) -> Result<usize, String> {
    let mut function_chunk = Chunk::new();

    let mut chunk_function_params: Vec<ChunkFunctionParam> = vec![];

    for param in params {
        // store all params from the stack into variables
        function_chunk.add_operation(&Operation::Store {
            name: param.name.clone(),
            accessors: vec![],
        });

        chunk_function_params.push(ChunkFunctionParam {
            constant: param.constant,
        });
    }

    chunk_function_params.reverse();

    let mut chunk_function_pre_param: Option<ChunkFunctionParam> = None;
    if let Some(pre_param) = pre_param {
        function_chunk.add_operation(&Operation::Store {
            name: pre_param.name.clone(),
            accessors: vec![],
        });

        chunk_function_pre_param = Some(ChunkFunctionParam {
            constant: pre_param.constant,
        });
    }

    // Index of this function once it's been added to the chunk (calculated in
    // advance)
    let index = chunk.functions.len();

    if name.is_some() {
        let name = name.clone().unwrap();

        // add mappings to the appropriate chunk fields based on whether it's a
        // value function or not (i.e. whether or not it has a pre-parameter)
        if pre_param.is_some() {
            let pre_param_type = pre_param.clone().unwrap().type_;

            // add mapping from this function name to the precalculated index
            chunk
                .value_function_index_from_name
                .insert((name.clone(), pre_param_type.clone()), index);

            // make the same mapping in the function's chunk to allow for
            // recursion (this index doesn't actually exist in the function's
            // chunk, but it will be executed from the parent chunk)
            function_chunk
                .value_function_index_from_name
                .insert((name, pre_param_type), index);
        } else {
            // add mapping from this function name to the precalculated index
            chunk.function_index_from_name.insert(name.clone(), index);

            // make the same mapping in the function's chunk to allow for
            // recursion (this index doesn't actually exist in the function's
            // chunk, but it will be executed from the parent chunk)
            function_chunk.function_index_from_name.insert(name, index);
        }

        // also add all the other function indexes to the function's chunk
        for (name, index) in chunk.function_index_from_name.clone() {
            function_chunk.function_index_from_name.insert(name, index);
        }
    }

    // compile function body
    compile_expression(&mut function_chunk, body, is_function)?;
    function_chunk.add_operation(&Operation::Halt);

    // add function to the chunk
    chunk.functions.push(ChunkFunction {
        chunk: function_chunk,
        pre_param: chunk_function_pre_param,
        parameters: chunk_function_params,
    });

    Ok(index)
}
