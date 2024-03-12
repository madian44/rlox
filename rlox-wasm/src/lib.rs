use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn example() {
    let mut chunk = rlox::Chunk::new();
    let reporter = rlox::DefaultReporter::new();

    let constant = chunk.add_constant(rlox::Value::Number(1.2));
    chunk.write_op_code(
        rlox::OpCode::Constant,
        rlox::Region::new_single_line(123, 0, 0),
    );
    chunk.write_byte(constant as u8, rlox::Region::new_single_line(123, 0, 0));

    let constant = chunk.add_constant(rlox::Value::Number(3.4));
    chunk.write_op_code(
        rlox::OpCode::Constant,
        rlox::Region::new_single_line(124, 0, 0),
    );
    chunk.write_byte(constant as u8, rlox::Region::new_single_line(124, 0, 0));

    chunk.write_op_code(rlox::OpCode::Add, rlox::Region::new_single_line(124, 0, 0));

    let constant = chunk.add_constant(rlox::Value::Number(5.6));
    chunk.write_op_code(
        rlox::OpCode::Constant,
        rlox::Region::new_single_line(124, 0, 0),
    );
    chunk.write_byte(constant as u8, rlox::Region::new_single_line(124, 0, 0));

    chunk.write_op_code(
        rlox::OpCode::Divide,
        rlox::Region::new_single_line(124, 0, 0),
    );
    chunk.write_op_code(
        rlox::OpCode::Negate,
        rlox::Region::new_single_line(124, 0, 0),
    );
    chunk.write_op_code(
        rlox::OpCode::Return,
        rlox::Region::new_single_line(125, 0, 0),
    );
    rlox::disassemble_chunk(&reporter, &chunk, "test chunk #1");

    let mut vm = rlox::Vm::new(&reporter);
    println!("Interpreting...");
    vm.interpret("");
}
