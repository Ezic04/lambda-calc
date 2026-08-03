use std::{borrow, iter};

use iced::{
    Element, Event, Length, Pixels, Point, Subscription, event, mouse,
    widget::{container, scrollable, table, text},
};
use sprs::CsMat;

use crate::app::Message;

use super::{dsl::ExprValue, lmatch};

struct Selection {
    selection_start: Option<Point>,
    selection_current: Option<Point>,
}

enum SelectionMessage {
    Mouse(mouse::Event),
}

fn subscription() -> Subscription<SelectionMessage> {
    event::listen_with(|event, _, _| match event {
        Event::Mouse(mouse_event) => Some(SelectionMessage::Mouse(mouse_event)),
        _ => None,
    })
}

#[derive(Debug, Clone)]
pub enum SpreadsheetMessage {
    ContentChanged {
        row: usize,
        col: usize,
        input: String,
    },
}

pub struct Spreadsheet {
    pub cells_content: CsMat<ExprValue>,
}

impl Default for Spreadsheet {
    fn default() -> Self {
        Self {
            cells_content: CsMat::zero((24, 12)),
        }
    }
}

impl Spreadsheet {
    pub fn update(&mut self, spreadsheet_message: SpreadsheetMessage) {
        match spreadsheet_message {
            SpreadsheetMessage::ContentChanged { row, col, input } => {
                self.cells_content.insert(row, col, ExprValue::Str(input));
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let fst_row_padding = [6, 10];
        let mk_cell = |row: usize, col: usize| {
            let val_str = self.cells_content.get(row, col).map_or(
                borrow::Cow::Borrowed(""),
                lmatch! {
                    ExprValue::Int(int) => borrow::Cow::Owned(int.to_string()),
                    ExprValue::Str(str) => borrow::Cow::Borrowed(str.as_str()),
                },
            );
            text(val_str)
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
}
