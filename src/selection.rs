use iced::Point;

pub struct Selection {
    selected: Option<(Point, Point)>,
    last_known: Option<Point>,
}

impl Selection {
    pub fn set_start(&mut self) {
        self.selected = self.last_known.map(|p| (p, p));
    }

    pub fn set_move(&mut self, position: Point) {
        self.last_known = Some(position);
        if let Some((_, end)) = &mut self.selected {
            *end = position
        }
    }

    pub fn set_end(&mut self) {
        self.selected = None;
    }

    pub fn get(&self) -> Option<(Point, Point)> {
        self.selected
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            selected: None,
            last_known: None,
        }
    }
}
