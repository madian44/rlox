pub struct Location {
    pub line: u16,
    pub start: u16,
    pub end: u16,
}

impl Location {
    pub fn new(line: u16, start: u16, end: u16) -> Self {
        Self { line, start, end }
    }

    pub fn has_same_line(&self, other: &Self) -> bool {
        self.line == other.line
    }
}
