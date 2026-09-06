use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index {
    pub row: usize,
    pub col: usize,
}

impl Index {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn map(&self, f: impl Fn(usize) -> usize) -> Self {
        Self {
            row: f(self.row),
            col: f(self.col),
        }
    }
}

impl Add<Offset> for Index {
    type Output = Index;

    fn add(self, rhs: Offset) -> Self::Output {
        let msg = "Index cannot be negative";
        Self::Output {
            row: self.row.checked_add_signed(rhs.row).expect(msg),
            col: self.col.checked_add_signed(rhs.col).expect(msg),
        }
    }
}

impl Sub for Index {
    type Output = Offset;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            row: self.row.wrapping_sub(rhs.row) as isize,
            col: self.col.wrapping_sub(rhs.col) as isize,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    pub row: isize,
    pub col: isize,
}

impl Default for Offset {
    fn default() -> Self {
        Self { row: 0, col: 0 }
    }
}
