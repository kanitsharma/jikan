use iced::pure::widget::Button;
use iced::pure::{button, container, row, text};

use crate::styles;

pub fn primary_button<'a, Message: 'a>(label: &str) -> Button<'a, Message> {
    button(
        container(text(label).size(18))
            .center_x()
            .center_y()
            .width(iced::Length::Units(100))
            .height(iced::Length::Units(30)),
    )
    .style(styles::Button::Primary)
}

pub fn secondary_button<'a, Message: 'a>(label: &str) -> Button<'a, Message> {
    button(row().padding(5).push(text(label).size(14))).style(styles::Button::Secondary)
}
