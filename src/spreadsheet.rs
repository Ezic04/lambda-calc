use std::{borrow, iter};

use iced::{
    Element, Event, Length, Pixels, Subscription, event,
    mouse::{self, Button},
    widget::{container, scrollable, table, text},
};
use sprs::CsMat;

use crate::selection::Selection;

use super::{dsl::ExprValue, lmatch};

#[derive(Debug, Clone)]
pub enum SpreadsheetMessage {
    ContentChanged {
        row: usize,
        col: usize,
        input: String,
    },
    Selection(mouse::Event),
}

pub struct Spreadsheet {
    pub cells_content: CsMat<ExprValue>,
    selection: Selection,
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
            SpreadsheetMessage::ContentChanged { row, col, input } => {
                self.cells_content.insert(row, col, ExprValue::Str(input));
            }
            SpreadsheetMessage::Selection(mouse) => {
                match mouse {
                    mouse::Event::ButtonPressed(Button::Left) => {
                        self.selection.set_start();
                    }
                    mouse::Event::CursorMoved { position } => {
                        self.selection.set_move(position);
                    }
                    mouse::Event::ButtonReleased(Button::Left) => {
                        self.selection.set_end();
                    }
                    _ => (),
                };
            }
        }
    }

    pub fn view(&self) -> Element<'_, SpreadsheetMessage> {
        let fst_row_padding = [6, 10];
        let mk_cell = |row: usize, col: usize| {
            let val_str = self.cells_content.get(row, col).map_or(
                borrow::Cow::Borrowed(""),
                lmatch! {
                    ExprValue::Int(int) => borrow::Cow::Owned(int.to_string()),
                    ExprValue::Str(str) => borrow::Cow::Borrowed(str.as_str()),
                },
            );
            let str = match self.selection.get() {
                Some((start, end)) => {
                    format!("{start}, {end}")
                }
                None => "none".to_string(),
            };
            // let w = text(val_str);
            text(str)
        };
        let fst_col = table::column(text!(""), |r| {
            container(text!("{r}").center().width(Length::Shrink)).padding(fst_row_padding)
        })
        .width(Length::Shrink);
        let cols = (0..self.cells_content.cols()).map(|c| {
            table::column(text!("{c}").center().width(Length::Fill), move |r| {
                mk_cell(r, c)
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

    pub fn subscription(&self) -> Subscription<SpreadsheetMessage> {
        event::listen_with(|event, _, _| match event {
            Event::Mouse(mouse_event) => Some(SpreadsheetMessage::Selection(mouse_event)),
            _ => None,
        })
    }
}
