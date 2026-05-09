use std::fmt;

pub struct Location {
    /// Path to the source code
    pub filepath: String,
    /// Current row in the file
    pub row: usize,
    /// Current column of the line
    pub col: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.filepath, self.row + 1, self.col + 1)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
