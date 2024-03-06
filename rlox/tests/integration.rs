use std::cell::RefCell;

#[test]
fn tests() {
    let tests: Vec<(Box<dyn Fn(&mut rlox::Chunk)>, Vec<&str>)> = vec![
        // let tests = vec![
        (
            Box::new(|chunk: &mut rlox::Chunk| {
                chunk.write_op_code(rlox::OpCode::Return, rlox::Location::new(1, 0, 0))
            }),
            vec!["0000    1 OP_RETURN"],
        ),
        (
            Box::new(|chunk: &mut rlox::Chunk| {
                chunk.write_op_code(rlox::OpCode::Constant, rlox::Location::new(1, 0, 0));
                let constant = chunk.add_constant(rlox::Value::Number(1.2));
                chunk.write_byte(constant as u8, rlox::Location::new(1, 0, 0));
                chunk.write_op_code(rlox::OpCode::Constant, rlox::Location::new(1, 0, 0));
                let constant = chunk.add_constant(rlox::Value::Number(2.4));
                chunk.write_byte(constant as u8, rlox::Location::new(1, 0, 0));
                chunk.write_op_code(rlox::OpCode::Return, rlox::Location::new(3, 0, 0));
            }),
            vec![
                "0000    1 OP_CONSTANT         0 1.20",
                "0002    | OP_CONSTANT         1 2.40",
                "0004    3 OP_RETURN",
            ],
        ),
    ];

    for (closure, expected_messages) in tests {
        let mut chunk = rlox::Chunk::new();
        let output = TestDebugOutput::new();

        closure(&mut chunk);

        rlox::disassemble_chunk(&output, &chunk, "test chunk");

        if (output.message_count() - 1) != expected_messages.len() {
            display_messages(&output, &expected_messages);
            panic!("Missmatched messages");
        }

        for (i, expected_message) in expected_messages.iter().enumerate() {
            if output.get_message(i + 1) != *expected_message {
                display_messages(&output, &expected_messages);
                panic!("Missmatched messages");
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
