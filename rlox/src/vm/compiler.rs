use crate::reporter::Reporter;
use crate::vm::{scanner::Scanner, token::TokenType};

pub fn compile(reporter: &dyn Reporter, source: &str) {
    let mut scanner = Scanner::new(source);
    let mut line: Option<u16> = None;
    loop {
        let token = scanner.scan_token();

        let mut message = "".to_string();
        if line.is_none() || line.unwrap() != token.location.start.line {
            line = Some(token.location.start.line);
            message.push_str(&format!("{:04} ", line.unwrap()));
        } else {
            message.push_str("   | ");
        }

        message.push_str(&format!("{:?}", token.token_type));
        reporter.add_message(&message);

        if token.token_type == TokenType::Eof {
            break;
        }
    }
}
