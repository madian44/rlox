use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Location {
    pub line: u16,
    pub offset: u16,
}

impl Location {
    pub fn new(line: u16, offset: u16) -> Self {
        Self { line, offset }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.offset)
    }
}

#[derive(PartialEq, Debug)]
pub struct Region {
    pub start: Location,
    pub end: Location,
}

impl Region {
    pub fn new(start_line: u16, start: u16, end_line: u16, end: u16) -> Self {
        Self {
            start: Location {
                line: start_line,
                offset: start,
            },
            end: Location {
                line: end_line,
                offset: end,
            },
        }
    }

    pub fn new_single_line(line: u16, start: u16, end: u16) -> Self {
        Self {
            start: Location {
                line,
                offset: start,
            },
            end: Location { line, offset: end },
        }
    }

    pub fn has_same_line(&self, other: &Self) -> bool {
        self.start.line == other.start.line && self.end.line == other.end.line
    }
}

impl Default for Region {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start.line == self.end.line {
            write!(
                f,
                "{}:{}-{}",
                self.start.line, self.start.offset, self.end.offset
            )
        } else {
            write!(
                f,
                "{}:{}-{}:{}",
                self.start.line, self.start.offset, self.end.line, self.end.offset
            )
        }
    }
}
