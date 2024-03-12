use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn example() {
    let mut chunk = rlox::Chunk::new();
    let output = rlox::DefaultDebugOutput::new();

    let constant = chunk.add_constant(rlox::Value::Number(1.2));
    chunk.write_op_code(rlox::OpCode::Constant, rlox::Location::new(123, 0, 0));
    chunk.write_byte(constant as u8, rlox::Location::new(123, 0, 0));

    let constant = chunk.add_constant(rlox::Value::Number(3.4));
    chunk.write_op_code(rlox::OpCode::Constant, rlox::Location::new(124, 0, 0));
    chunk.write_byte(constant as u8, rlox::Location::new(124, 0, 0));

    chunk.write_op_code(rlox::OpCode::Add, rlox::Location::new(124, 0, 0));

    let constant = chunk.add_constant(rlox::Value::Number(5.6));
    chunk.write_op_code(rlox::OpCode::Constant, rlox::Location::new(124, 0, 0));
    chunk.write_byte(constant as u8, rlox::Location::new(124, 0, 0));

    chunk.write_op_code(rlox::OpCode::Divide, rlox::Location::new(124, 0, 0));
    chunk.write_op_code(rlox::OpCode::Negate, rlox::Location::new(124, 0, 0));
    chunk.write_op_code(rlox::OpCode::Return, rlox::Location::new(125, 0, 0));
    rlox::disassemble_chunk(&output, &chunk, "test chunk #1");

    let mut vm = rlox::Vm::new(chunk);
    println!("Interpreting...");
    vm.interpret();
}
