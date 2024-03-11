use crate::chunk;

pub trait DebugOutput {
    fn write(&self, message: &str);
}

#[derive(Default)]
pub struct DefaultDebugOutput {}

impl DefaultDebugOutput {
    pub fn new() -> Self {
        Self {}
    }
}

impl DebugOutput for DefaultDebugOutput {
    fn write(&self, message: &str) {
        println!("{}", message);
    }
}

pub fn disassemble_chunk(output: &dyn DebugOutput, chunk: &chunk::Chunk, name: &str) {
    output.write(&format!("=== {name} ==="));

    let mut index = 0;
    while index < chunk.code.len() {
        index = disassemble_instruction(output, chunk, index)
    }
}

pub fn disassemble_instruction(
    output: &dyn DebugOutput,
    chunk: &chunk::Chunk,
    index: usize,
) -> usize {
    let header = format!("{index:04}");
    let line = if index > 0 && chunk.locations[index].has_same_line(&chunk.locations[index - 1]) {
        "   |".to_string()
    } else {
        format!("{:4}", chunk.locations[index].line)
    };
    let op_code = byte_to_op_code(chunk.code[index]);
    let (increment, content) = match op_code {
        Some(chunk::OpCode::Return) => simple_instruction("OP_RETURN"),
        Some(chunk::OpCode::Constant) => constant_instruction("OP_CONSTANT", chunk, index),
        Some(chunk::OpCode::Negate) => simple_instruction("OP_NEGATE"),
        Some(chunk::OpCode::Add) => simple_instruction("OP_ADD"),
        Some(chunk::OpCode::Subtract) => simple_instruction("OP_SUBTRACT"),
        Some(chunk::OpCode::Multiply) => simple_instruction("OP_MULTIPLY"),
        Some(chunk::OpCode::Divide) => simple_instruction("OP_DIVIDE"),
        None => (1, format!("Unknown op_code {}", chunk.code[index])),
    };
    output.write(&format!("{header} {line} {content}"));
    index + increment
}

fn byte_to_op_code(byte: u8) -> Option<chunk::OpCode> {
    // match byte {
    //     x if x == chunk::OpCode::Constant as u8 => Some(chunk::OpCode::Constant),
    //     x if x == chunk::OpCode::Return as u8 => Some(chunk::OpCode::Return),
    //     _ => None,
    // }
    let op_code: chunk::OpCode = unsafe { std::mem::transmute(byte) };
    Some(op_code)
}

fn simple_instruction(name: &str) -> (usize, String) {
    (1, name.to_string())
}

fn constant_instruction(name: &str, chunk: &chunk::Chunk, index: usize) -> (usize, String) {
    let constant_index = chunk.code[index + 1];
    let constant = chunk
        .constants
        .get(constant_index as usize)
        .map_or_else(|| "".to_string(), |o| o.to_string());
    (2, format!("{name:<16} {constant_index:4} {constant}"))
}
