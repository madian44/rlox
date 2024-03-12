#[cfg(feature = "debug_trace_execution")]
use crate::debug::{disassemble_instruction, DebugOutput, DefaultDebugOutput};
use crate::{chunk, value};

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

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct Vm {
    chunk: chunk::Chunk,
    ip: *const u8,
    stack: Vec<value::Value>,
}

impl Vm {
    pub fn new(chunk: chunk::Chunk) -> Self {
        Self {
            chunk,
            ip: std::ptr::null_mut(),
            stack: Vec::with_capacity(STACK_MAX),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
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
        let index = self.read_byte() as usize;
        self.chunk.constants.get(index).cloned()
    }

    #[inline(always)]
    fn read_op_code(&mut self) -> chunk::OpCode {
        let op_code: chunk::OpCode = unsafe {
            // DANGER!
            #[cfg(feature = "debug_trace_execution")]
            {
                let output = DefaultDebugOutput::new();
                output.write(&format!("          {}", self.get_stack_string()));
                let index = self.ip.offset_from(self.chunk.code.as_ptr());
                disassemble_instruction(&output, &self.chunk, index as usize);
            }
            std::mem::transmute(self.read_byte())
        };
        op_code
    }
}
