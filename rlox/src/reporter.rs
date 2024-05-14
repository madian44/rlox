use crate::location;
use std::cell::RefCell;

pub trait Reporter {
    fn add_diagnostic(&self, location: &location::Region, message: &str);

    fn add_message(&self, message: &str);

    fn has_diagnostics(&self) -> bool;
}

pub struct DefaultReporter {
    has_errors: RefCell<bool>,
}

impl DefaultReporter {
    pub fn new() -> Self {
        Self {
            has_errors: RefCell::new(false),
        }
    }
}

impl Default for DefaultReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for DefaultReporter {
    fn add_diagnostic(&self, location: &location::Region, message: &str) {
        println!("[{}]: {}", location, message);
        *self.has_errors.borrow_mut() = true;
    }

    fn add_message(&self, message: &str) {
        println!("{}", message);
    }

    fn has_diagnostics(&self) -> bool {
        *self.has_errors.borrow()
    }
}
