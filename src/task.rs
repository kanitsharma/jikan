use std::usize;

use iced::{
    pure::{row, text, widget::Container, Element},
    Length, Space,
};

use crate::TodoMessage;
#[path = "components/buttons.rs"] mod buttons;

#[derive(Default, Clone)]
pub struct Task {
    title: String,
    index: usize,
}

impl Task {
    pub fn new(title: String, index: usize) -> Self {
        Task { title, index }
    }

    pub fn view<'a>(&self) -> Element<'a, TodoMessage> {
        let task = text(self.title.clone());
        let delete_todo = buttons::secondary_button("Delete").on_press(TodoMessage::DeleteTodo(self.index));

        Container::new(
            row()
                .width(Length::Fill)
                .align_items(iced::Alignment::Center)
                .push(task)
                .push(Space::with_width(Length::Fill))
                .push(delete_todo),
        )
        .width(Length::Fill)
        .center_y()
        .into()
    }
}
