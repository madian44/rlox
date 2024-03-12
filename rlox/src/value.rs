use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone)]
pub enum Value {
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Value::Number(value) => write!(f, "{value:.2}"),
        }
    }
}

#[derive(Default)]
pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn write(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }

    pub fn free(self) -> Self {
        Self::new()
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.values.get(index)
    }
}
