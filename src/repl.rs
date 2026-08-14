use composing::compose_fn;
use iced::{
    Length,
    widget::{column, text, text_editor},
};
use std::convert::identity;

use super::dsl::ExprValue;

#[derive(Debug, Clone)]
pub enum ReplMessage {
    EditorAction(text_editor::Action),
}

pub struct Repl {
    content: text_editor::Content,
    pub expr_res: Result<ExprValue, String>,
}

impl Repl {
    pub fn view(&self) -> iced::Element<'_, ReplMessage> {
        let expr_val_str = self
            .expr_res
            .clone()
            .map_or_else(identity, |v| v.to_string());
        column![
            text_editor(&self.content)
                .on_action(ReplMessage::EditorAction)
                .placeholder("repl")
                .height(Length::Fill),
            text!("{expr_val_str}")
                .size(24)
                .center()
                .width(Length::Fill)
                .height(Length::Fill),
        ]
        .height(Length::FillPortion(1))
        .into()
    }

    pub fn update(&mut self, repl_message: ReplMessage) {
        match repl_message {
            ReplMessage::EditorAction(action) => {
                self.content.perform(action);
            }
        }
    }

    pub fn content_text(&self) -> String {
        self.content.text()
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self {
            content: text_editor::Content::new(),
            expr_res: Err("nothing yet".to_string()),
        }
    }
}
