use crate::spreadsheet::index::Offset;

use super::index::Index;

#[derive(Debug)]
pub struct Selection {
    pub start: Index,
    offset: Offset,
    current: Option<Index>,
    is_pressed: bool,
}

impl Selection {
    pub fn is_in_range(&self, index: Index) -> bool {
        let end = self.start + self.offset;
        (usize::min(self.start.row, end.row)..=usize::max(self.start.row, end.row))
            .contains(&index.row)
            && (usize::min(self.start.col, end.col)..=usize::max(self.start.col, end.col))
                .contains(&index.col)
    }

    pub fn on_enter(&mut self, index: Index) {
        self.current = Some(index);
        if self.is_pressed {
            self.offset = index - self.start
        }
    }

    pub fn on_exit(&mut self, index: Index) {
        if self.current == Some(index) {
            self.current = None;
        }
    }

    pub fn on_press(&mut self) {
        assert!(!self.is_pressed);
        self.is_pressed = true;
        if let Some(current) = self.current {
            self.start = current;
            self.offset = Offset { row: 0, col: 0 }
        }
    }
    pub fn on_release(&mut self) {
        assert!(self.is_pressed);
        self.is_pressed = false;
    }
}

impl Default for Selection {
    fn default() -> Self {
        let default_idx = Index { row: 0, col: 0 };
        Self {
            start: default_idx,
            offset: Offset::default(),
            current: None,
            is_pressed: false,
        }
    }
}
