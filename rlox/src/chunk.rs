use crate::{location, value};

#[repr(u8)]
pub enum OpCode {
    Constant = 1,
    Add = 2,
    Subtract = 3,
    Multiply = 4,
    Divide = 5,
    Negate = 6,
    Return = 7,
}

#[derive(Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: value::ValueArray,
    pub locations: Vec<location::Region>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: value::ValueArray::new(),
            locations: vec![],
        }
    }

    pub fn write_op_code(&mut self, op_code: OpCode, location: location::Region) {
        self.code.push(op_code as u8);
        self.locations.push(location);
    }

    pub fn write_byte(&mut self, byte: u8, location: location::Region) {
        self.code.push(byte);
        self.locations.push(location);
    }

    pub fn free(self) -> Self {
        Self::new()
    }

    pub fn add_constant(&mut self, value: value::Value) -> usize {
        self.constants.write(value)
    }
    
    pub fn has_code(&self) -> bool {
        !self.code.is_empty()
    }
}
