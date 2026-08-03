use iced::{
    Length, Task,
    widget::{column, scrollable, text},
};

use crate::{
    dsl::{ExprValue, eval_expr},
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
    pub fn update(state: &mut Self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Repl(repl_message) => {
                state.repl.update(repl_message);
                return Task::perform(
                    eval_expr(state.vm.clone(), state.repl.content_text()),
                    Message::ExprEvaluated,
                );
            }
            Message::Spreadsheet(spreadsheet_message) => {
                state.spreadsheet.update(spreadsheet_message);
            }
            Message::ExprEvaluated(expr_res) => state.repl.expr_res = expr_res,
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
            self.spreadsheet.view(),
            self.repl.view()
        ]
        .into()
    }
}
