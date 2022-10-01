use iced::pure::widget::Button;
use iced::pure::{button, column, container, row, text, text_input, Element, Sandbox};
use iced::Settings;
// mod numeric_input;
mod component;

fn main() -> Result<(), iced::Error> {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {
    todo_list: Vec<String>,
    current_task: String,
}

#[derive(Debug, Clone)]
pub enum TodoMessage {
    AddTodo(String),
    DeleteTodo(usize),
    CurrentTodo(String),
}

impl Sandbox for App {
    type Message = TodoMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("TodoList")
    }

    fn update(&mut self, message: TodoMessage) {
        match message {
            TodoMessage::AddTodo(x) => {
                self.todo_list.push(x);
                self.current_task = String::new();
            }
            TodoMessage::DeleteTodo(i) => {
                self.todo_list.swap_remove(i);
            }
            TodoMessage::CurrentTodo(input) => self.current_task = input,
        }
    }

    fn view(&self) -> Element<TodoMessage> {
        let tasks =
            self.todo_list
                .iter()
                .enumerate()
                .fold(column().spacing(20), |col, (i, task)| {
                    let task = text(task);
                    let delete_todo =
                        secondaryy_button("Delete").on_press(TodoMessage::DeleteTodo(i));
                    col.push(
                        row()
                            .align_items(iced::Alignment::Center)
                            .spacing(10)
                            .push(task)
                            .push(delete_todo),
                    )
                });

        let input = text_input(
            "Write your task here",
            &self.current_task,
            TodoMessage::CurrentTodo,
        )
        .style(style::Input::Default)
        .padding(10)
        .on_submit(TodoMessage::AddTodo(self.current_task.clone()));

        let add_todo =
            primary_button("Add Task").on_press(TodoMessage::AddTodo(self.current_task.clone()));

        let component = container(component::MainPage::new().view());

        let content = column()
            .padding(20)
            .align_items(iced::Alignment::Start)
            .push(container(tasks).padding(30))
            // .push(numeric_input::numeric_input(self.value, TodoMessage::NumericInputChanged))
            .push(component)
            .push(
                container(
                    row()
                        .push(
                            container(input)
                                .width(iced::Length::Fill)
                                .center_y()
                                .align_y(iced::alignment::Vertical::Top),
                        )
                        .push(add_todo)
                        .align_items(iced::Alignment::Start)
                        .spacing(10),
                )
                .height(iced::Length::Fill)
                .align_y(iced::alignment::Vertical::Bottom),
            );

        container(content)
            .style(style::Container::Default)
            .center_x()
            .align_y(iced::alignment::Vertical::Bottom)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn primary_button<'a, Message: 'a>(label: &str) -> Button<'a, Message> {
    button(
        container(text(label).size(18))
            .center_x()
            .center_y()
            .width(iced::Length::Units(100))
            .height(iced::Length::Units(30)),
    )
    .style(style::Button::Primary)
}

fn secondaryy_button<'a, Message: 'a>(label: &str) -> Button<'a, Message> {
    button(row().padding(5).push(text(label).size(14))).style(style::Button::Secondary)
}

mod style {
    use iced::{button, container, text_input};
    use iced::{Background, Color};

    pub enum Button {
        Primary,
        Secondary,
    }

    pub enum Container {
        Default,
    }

    pub enum Input {
        Default,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgba8(227, 66, 50, 1.0),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 6.0,
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                ..self.active()
            }
        }
    }

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
                text_color: Color::WHITE.into(),
                ..container::Style::default()
            }
        }
    }

    impl text_input::StyleSheet for Input {
        fn active(&self) -> iced::text_input::Style {
            text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border_width: 0.5,
                border_color: Color::from_rgba8(0xEE, 0xEE, 0xEE, 0.5),
                border_radius: 6.0,
                ..text_input::Style::default()
            }
        }

        fn focused(&self) -> iced::text_input::Style {
            text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border_width: 0.5,
                border_color: Color::from_rgba8(0xEE, 0xEE, 0xEE, 0.5),
                border_radius: 6.0,
                ..text_input::Style::default()
            }
        }

        fn hovered(&self) -> iced::text_input::Style {
            text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border_width: 0.5,
                border_color: Color::from_rgba8(0xEE, 0xEE, 0xEE, 0.5),
                border_radius: 6.0,
                ..text_input::Style::default()
            }
        }

        fn selection_color(&self) -> Color {
            Color::from_rgb8(0xEE, 0xEE, 0xEE)
        }

        fn placeholder_color(&self) -> Color {
            Color::from_rgb8(0xEE, 0xEE, 0xEE)
        }

        fn value_color(&self) -> Color {
            Color::WHITE
        }
    }
}
