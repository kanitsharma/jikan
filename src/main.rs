use iced::pure::widget::{Button, Container};
use iced::pure::{button, column, container, row, text, Element, Sandbox, text_input};
use iced::Settings;

fn main() -> Result<(), iced::Error> {
    Counter::run(Settings::default())
}

#[derive(Clone)]
struct Counter {
    todo_list: Vec<String>,
    current_task: String
}

#[derive(Debug, Clone)]
enum TodoMessage {
    AddTodo(String),
    DeleteTodo(usize),
    CurrentTodo(String)
}

impl Sandbox for Counter {
    type Message = TodoMessage;

    fn new() -> Self {
        Counter { todo_list: vec![], current_task: String::new() }
    }

    fn title(&self) -> String {
        String::from("TodoList")
    }

    fn update(&mut self, message: TodoMessage) {
        match message {
            TodoMessage::AddTodo(x) => self.todo_list.push(x),
            TodoMessage::DeleteTodo(i) => {
                self.todo_list.swap_remove(i);
            }
            TodoMessage::CurrentTodo(input) => {
                self.current_task = input
            }
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

        let input = text_input("Write your task here", &self.current_task, TodoMessage::CurrentTodo);

        let add_todo =
            primary_button("Add Task").on_press(TodoMessage::AddTodo(self.current_task.clone()));

        let content = column().padding(20).push(tasks).push(add_todo).push(input);

        container(content)
            .style(style::Container::Default)
            .center_x()
            .center_y()
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
    use iced::{button, container};
    use iced::{Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    pub enum Container {
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
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
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
}
