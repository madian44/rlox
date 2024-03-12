use rlox::DebugOutput;
use std::cell::RefCell;

type TestCode = Box<dyn Fn(&mut rlox::Chunk)>;

#[test]
fn tests() {
    let tests: Vec<(TestCode, Vec<&str>)> = vec![
        (
            Box::new(|chunk: &mut rlox::Chunk| {
                chunk.write_op_code(rlox::OpCode::Return, rlox::Location::new(1, 0, 0))
            }),
            vec!["0000    1 OP_RETURN", ""],
        ),
        (
            Box::new(|chunk: &mut rlox::Chunk| {
                let constant = chunk.add_constant(rlox::Value::Number(1.2));
                chunk.write_op_code(rlox::OpCode::Constant, rlox::Location::new(1, 0, 0));
                chunk.write_byte(constant as u8, rlox::Location::new(1, 0, 0));

                let constant = chunk.add_constant(rlox::Value::Number(3.4));
                chunk.write_op_code(rlox::OpCode::Constant, rlox::Location::new(1, 0, 0));
                chunk.write_byte(constant as u8, rlox::Location::new(1, 0, 0));

                chunk.write_op_code(rlox::OpCode::Add, rlox::Location::new(1, 0, 0));

                let constant = chunk.add_constant(rlox::Value::Number(5.6));
                chunk.write_op_code(rlox::OpCode::Constant, rlox::Location::new(2, 0, 0));
                chunk.write_byte(constant as u8, rlox::Location::new(2, 0, 0));

                chunk.write_op_code(rlox::OpCode::Divide, rlox::Location::new(2, 0, 0));
                chunk.write_op_code(rlox::OpCode::Negate, rlox::Location::new(2, 0, 0));
                chunk.write_op_code(rlox::OpCode::Return, rlox::Location::new(3, 0, 0));
            }),
            vec![
                "0000    1 OP_CONSTANT         0 1.20",
                "0002    | OP_CONSTANT         1 3.40",
                "0004    | OP_ADD",
                "0005    2 OP_CONSTANT         2 5.60",
                "0007    | OP_DIVIDE",
                "0008    | OP_NEGATE",
                "0009    3 OP_RETURN",
                "[-0.82]",
            ],
        ),
    ];

    for (closure, expected_messages) in tests {
        let mut chunk = rlox::Chunk::new();
        let output = TestDebugOutput::new();

        closure(&mut chunk);

        rlox::disassemble_chunk(&output, &chunk, "test chunk");
        let mut vm = rlox::Vm::new(chunk);
        vm.interpret();
        output.write(&vm.get_stack_string());

        if (output.message_count() - 1) != expected_messages.len() {
            display_messages(&output, &expected_messages);
            panic!("Mismatched messages");
        }

        for (i, expected_message) in expected_messages.iter().enumerate() {
            if output.get_message(i + 1) != *expected_message {
                display_messages(&output, &expected_messages);
                panic!("Mismatched messages");
            }
        }
    }
}

fn display_messages(output: &TestDebugOutput, expected_messages: &[&str]) {
    println!("Actual messages:");
    output.display_messages();
    println!("Expected messages:");
    expected_messages.iter().for_each(|m| println!("{}", m));
}

struct TestDebugOutput {
    messages: RefCell<Vec<String>>,
}

impl TestDebugOutput {
    fn new() -> Self {
        Self {
            messages: RefCell::new(Vec::new()),
        }
    }

    fn message_count(&self) -> usize {
        self.messages.borrow().len()
    }

    fn display_messages(&self) {
        self.messages
            .borrow()
            .iter()
            .for_each(|m| println!("{}", m));
    }

    fn get_message(&self, i: usize) -> String {
        self.messages.borrow().get(i).unwrap().clone()
    }
}

impl rlox::DebugOutput for TestDebugOutput {
    fn write(&self, line: &str) {
        self.messages.borrow_mut().push(line.to_string());
    }
}
