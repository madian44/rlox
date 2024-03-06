use rlox::Location;

fn main() {
    println!("Hello, world!");

    let mut chunk = rlox::Chunk::new();
    let output = rlox::DefaultDebugOutput::new();

    let constant = chunk.add_constant(rlox::Value::Number(1.2));
    chunk.write_op_code(rlox::OpCode::Constant, Location::new(123, 0, 0));
    chunk.write_byte(constant as u8, Location::new(123, 0, 0));
    chunk.write_op_code(rlox::OpCode::Return, Location::new(124, 0, 0));
    rlox::disassemble_chunk(&output, &chunk, "test chunk #1");
    chunk.free();
}
