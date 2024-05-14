mod compiler;
mod result;
mod scanner;
mod token;

#[cfg(feature = "debug_trace_execution")]
use crate::debug::disassemble_instruction;
use crate::{chunk, reporter::Reporter, value};

use crate::vm::compiler::compile;
pub use crate::vm::result::InterpretResult;

const STACK_MAX: usize = 256;

macro_rules! bin_op {
    ($stack:expr, $op:tt) => {
        {
            if let Some(value::Value::Number(b)) = $stack.pop() {
                if let Some(value::Value::Number(a)) = $stack.pop() {
                    let r = a $op b;
                    $stack.push(value::Value::Number(r));
                }
            }
        }
    }
}

pub struct Vm<'a> {
    reporter: &'a dyn Reporter,
    ip: *const u8,
    stack: Vec<value::Value>,

    #[cfg(feature = "debug_chunk")]
    chunk: chunk::Chunk,
}

impl<'a> Vm<'a> {
    pub fn new(reporter: &'a dyn Reporter) -> Self {
        Self {
            reporter,
            ip: std::ptr::null_mut(),
            stack: Vec::with_capacity(STACK_MAX),
            #[cfg(feature = "debug_chunk")]
            chunk: chunk::Chunk::new()
        }
    }
    
    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        compile(self.reporter, source);
        //self.ip = self.chunk.code.as_ptr();
        //self.run()
        InterpretResult::Ok
    }

    #[cfg(feature = "debug_chunk")]
    pub fn test_interpret_chunk(&mut self, chunk: chunk::Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = self.chunk.code.as_ptr();
        self.run()
    }

    pub fn get_stack_string(&self) -> String {
        let mut result = "".to_string();
        self.stack
            .iter()
            .for_each(|v| result.push_str(&format!("[{v}]")));
        result
    }

    fn run(&mut self) -> InterpretResult {
        let mut op_code: chunk::OpCode;
        loop {
            op_code = self.read_op_code();
            match op_code {
                chunk::OpCode::Negate => self.negate(),
                chunk::OpCode::Add => bin_op!(self.stack, +),
                chunk::OpCode::Subtract => bin_op!(self.stack, -),
                chunk::OpCode::Multiply => bin_op!(self.stack, *),
                chunk::OpCode::Divide => bin_op!(self.stack, /),
                chunk::OpCode::Return => {
                    return InterpretResult::Ok;
                }
                chunk::OpCode::Constant => {
                    if let Some(constant) = self.read_constant() {
                        self.stack.push(constant);
                    }
                }
            }
        }
    }

    fn negate(&mut self) {
        if let Some(value::Value::Number(value)) = self.stack.pop() {
            self.stack.push(value::Value::Number(-value));
        }
    }

    #[inline(always)]
    fn read_byte(&mut self) -> u8 {
        let byte: u8;
        unsafe {
            // DANGER!
            byte = *self.ip;
            self.ip = self.ip.add(1);
        }
        byte
    }

    #[inline(always)]
    fn read_constant(&mut self) -> Option<value::Value> {
        #[cfg(feature = "debug_chunk")]
        {
            let index = self.read_byte() as usize;
            return self.chunk.constants.get(index).cloned();
        } 
        #[cfg(not(feature = "debug_chunk"))]
        None
    }

    #[inline(always)]
    fn read_op_code(&mut self) -> chunk::OpCode {
        let op_code: chunk::OpCode = unsafe {
            // DANGER!
            #[cfg(feature = "debug_trace_execution")]
            {
                #[cfg(feature = "debug_chunk")]
                if self.chunk.has_code() {
                    self.reporter.add_message(&format!("          {}", self.get_stack_string()));
                    let index = self.ip.offset_from(self.chunk.code.as_ptr());
                    disassemble_instruction(self.reporter, &self.chunk, index as usize);
                }
            }
            std::mem::transmute(self.read_byte())
        };
        op_code
    }
}
