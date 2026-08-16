use iced::{
    Length, Subscription, Task,
    widget::{column, text},
};

use crate::{
    dsl::ExprValue,
    repl::{Repl, ReplMessage},
    spreadsheet::{Spreadsheet, SpreadsheetMessage},
};

#[derive(Debug, Clone)]
pub enum Message {
    Repl(ReplMessage),
    Spreadsheet(SpreadsheetMessage),
    ExprEvaluated(Result<ExprValue, String>),
}

pub struct App {
    vm: gluon::RootedThread,
    spreadsheet: Spreadsheet,
    repl: Repl,
}

impl Default for App {
    fn default() -> Self {
        Self {
            vm: gluon::new_vm(),
            spreadsheet: Spreadsheet::default(),
            repl: Repl::default(),
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Repl(repl_message) => {
                return self
                    .repl
                    .update(repl_message, self.vm.clone())
                    .map(Message::ExprEvaluated);
            }
            Message::Spreadsheet(spreadsheet_message) => {
                self.spreadsheet.update(spreadsheet_message);
            }
            Message::ExprEvaluated(expr_res) => {
                self.repl.on_evaluated(&expr_res);
                self.spreadsheet.on_evaluated(&expr_res);
            }
        }
        iced::Task::none()
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        column![
            text!("Lambda Calc")
                .size(24)
                .center()
                .width(Length::Fill)
                .height(Length::Shrink),
            self.spreadsheet.view().map(Message::Spreadsheet),
            self.repl.view().map(Message::Repl)
        ]
        .into()
    }
    pub fn subscription(&self) -> Subscription<Message> {
        self.spreadsheet.subscription().map(Message::Spreadsheet)
    }
}
