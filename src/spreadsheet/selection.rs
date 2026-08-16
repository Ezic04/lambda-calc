use super::index::Index;

pub struct Selection {
    pub start: Index,
    pub end: Index,
    current: Option<Index>,
    is_pressed: bool,
}

impl Selection {
    pub fn on_enter(&mut self, index: Index) {
        self.current = Some(index);
        if self.is_pressed {
            self.end = index
        }
    }

    pub fn on_exit(&mut self) {
        self.current = None
    }

    pub fn on_press(&mut self) {
        assert!(!self.is_pressed);
        self.is_pressed = true;
        if let Some(current) = self.current {
            self.start = current;
            self.end = current;
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
            end: default_idx,
            current: None,
            is_pressed: false,
        }
    }
}
