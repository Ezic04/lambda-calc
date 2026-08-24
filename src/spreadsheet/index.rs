#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index {
    pub row: usize,
    pub col: usize,
}

impl Index {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn is_in_range(&self, start: Index, end: Index) -> bool {
        start.row <= self.row && self.row <= end.row && start.col <= self.col && self.col <= end.col
    }
}
