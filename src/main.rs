use gluon::ThreadExt;
use iced::{
    Length, Pixels, Task,
    widget::{column, container, scrollable, table, text, text_editor, text_input},
};
use sprs::CsMat;
use std::iter;

pub fn main() -> iced::Result {
    iced::run(Spreadsheet::update, Spreadsheet::view)
}

#[derive(Debug, Clone)]
enum ExprType {
    Int(i32),
    Error(String),
}

struct Spreadsheet {
    vm: gluon::RootedThread,
    cells_content: CsMat<String>,
    repl_content: text_editor::Content,
    expr_val: ExprType,
}
impl Default for Spreadsheet {
    fn default() -> Self {
        Self {
            vm: gluon::new_vm(),
            cells_content: CsMat::zero((24, 12)),
            repl_content: text_editor::Content::new(),
            expr_val: ExprType::Int(0),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged { row: usize, col: usize, val: String },
    ReplEdit(text_editor::Action),
    ExprEvaluated(ExprType),
}

fn eval_expr(vm: gluon::RootedThread, expr: String) -> impl Future<Output = ExprType> + Send {
    async move {
        match vm.run_expr_async::<i32>("example", expr.as_str()).await {
            Ok(val) => ExprType::Int(val.0),
            Err(err) => ExprType::Error(err.to_string()),
        }
    }
}

impl Spreadsheet {
    fn view(&self) -> iced::Element<'_, Message> {
        let fst_row_padding = [6, 10];
        let expr_val_str = &match &self.expr_val {
            ExprType::Int(x) => &x.to_string(),
            ExprType::Error(err) => err.as_str(),
        };
        let mk_cell = |row: usize, col: usize| {
            let val = self.cells_content.get(row, col);
            text_input("*", val.map_or("", |val| val.as_str()))
                .on_input(move |val| Message::ContentChanged { row, col, val })
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
        scrollable(column![
            text!("Lambda Calc").size(24).center().width(Length::Fill),
            table::table(
                iter::once(fst_col).chain(cols),
                (0..self.cells_content.rows()).into_iter()
            )
            .padding(Pixels::ZERO)
            .width(Length::Fill),
            text_editor(&self.repl_content).on_action(Message::ReplEdit),
            text!("{expr_val_str}")
                .size(24)
                .center()
                .width(Length::Fill),
        ])
        .into()
    }

    fn update(state: &mut Self, message: Message) -> iced::Task<Message> {
        match message {
            Message::ContentChanged { row, col, val } => {
                format!("({row},{col}): {val}\n").chars().for_each(|char| {
                    state
                        .repl_content
                        .perform(text_editor::Action::Edit(text_editor::Edit::Insert(char)));
                });
                state.cells_content.insert(row, col, val);
            }
            Message::ReplEdit(action) => {
                state.repl_content.perform(action);
                return Task::perform(
                    eval_expr(state.vm.clone(), state.repl_content.text()),
                    Message::ExprEvaluated,
                );
            }
            Message::ExprEvaluated(val) => state.expr_val = val,
        }
        iced::Task::none()
    }
}
