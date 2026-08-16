mod index;
pub mod selection;

use std::{borrow, iter};

use iced::{
    Color, Element, Event, Length, Pixels, Subscription, event,
    mouse::{self, Button},
    widget::{container, mouse_area, scrollable, table, text},
};
use sprs::CsMat;

use super::{dsl::ExprValue, lmatch};
use crate::spreadsheet::{index::Index, selection::Selection};

#[derive(Debug, Clone)]
pub enum SpreadsheetMessage {
    ContentChanged { index: Index, input: String },
    MouseEvent(mouse::Event),
    EnteredCell(Index),
    ExitedCell,
}

pub struct Spreadsheet {
    pub cells_content: CsMat<ExprValue>,
    pub selection: Selection,
}

impl Default for Spreadsheet {
    fn default() -> Self {
        Self {
            cells_content: CsMat::zero((24, 12)),
            selection: Selection::default(),
        }
    }
}

impl Spreadsheet {
    pub fn update(&mut self, spreadsheet_message: SpreadsheetMessage) {
        match spreadsheet_message {
            SpreadsheetMessage::ContentChanged { index, input } => {
                self.cells_content
                    .insert(index.row, index.col, ExprValue::Str(input));
            }
            SpreadsheetMessage::MouseEvent(mouse) => {
                match mouse {
                    mouse::Event::ButtonPressed(Button::Left) => self.selection.on_press(),
                    mouse::Event::ButtonReleased(Button::Left) => self.selection.on_release(),
                    _ => (),
                };
            }
            SpreadsheetMessage::EnteredCell(index) => self.selection.on_enter(index),
            SpreadsheetMessage::ExitedCell => self.selection.on_exit(),
        }
    }

    pub fn view(&self) -> Element<'_, SpreadsheetMessage> {
        let cell_padding = [6, 10];
        let mk_cell = |index: Index| {
            let content_str = self.cells_content.get(index.row, index.col).map_or(
                borrow::Cow::Borrowed("?"),
                lmatch! {
                    ExprValue::Int(int) => borrow::Cow::Owned(int.to_string()),
                    ExprValue::Str(str) => borrow::Cow::Borrowed(str.as_str()),
                },
            );
            let color = if index.is_in_range(self.selection.start, self.selection.end) {
                Color::BLACK
            } else {
                Color::WHITE
            };
            let text = text(content_str).color(color).width(Length::Fill);
            let container = container(text).padding(cell_padding);
            mouse_area(container)
                .on_enter(SpreadsheetMessage::EnteredCell(index))
                .on_exit(SpreadsheetMessage::ExitedCell)
        };
        let fst_col = table::column(text!(""), |row| {
            container(text!("{row}").center().width(Length::Shrink)).padding(cell_padding)
        })
        .width(Length::Shrink);
        let cols = (0..self.cells_content.cols()).map(|col| {
            table::column(text!("{col}").center().width(Length::Fill), move |row| {
                mk_cell(Index::new(row, col))
            })
            .width(Length::Fill)
        });
        let table = table::table(
            iter::once(fst_col).chain(cols),
            (0..self.cells_content.rows()).into_iter(),
        )
        .padding(Pixels::ZERO);
        scrollable(table).height(Length::FillPortion(3)).into()
    }

    pub fn on_evaluated(&mut self, expr_res: &Result<ExprValue, String>) {
        let idx = self.selection.start;
        if let Ok(expr) = expr_res {
            self.cells_content.insert(idx.row, idx.col, expr.clone());
        }
    }

    pub fn subscription(&self) -> Subscription<SpreadsheetMessage> {
        event::listen_with(|event, _, _| match event {
            Event::Mouse(mouse_event) => Some(SpreadsheetMessage::MouseEvent(mouse_event)),
            _ => None,
        })
    }
}
