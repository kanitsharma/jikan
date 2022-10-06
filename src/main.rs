use iced::pure::widget::Button;
use iced::pure::{
    button, column, container, progress_bar, row, text, text_input, Element, Sandbox,
};
use iced::Settings;

fn main() -> Result<(), iced::Error> {
    Counter::run(Settings::default())
}

#[derive(Clone)]
struct Counter {
    todo_list: Vec<Todo>,
    current_task: Todo,
}

#[derive(Debug, Clone, Default)]
struct Todo {
    message: String,
    time: f32,
}

#[derive(Debug, Clone)]
enum TodoMessage {
    AddTodo(Todo),
    DeleteTodo(usize),
    CurrentTodoMessage(String),
    CurrentTodoTimer(f32),
}

impl Sandbox for Counter {
    type Message = TodoMessage;

    fn new() -> Self {
        Counter {
            todo_list: vec![],
            current_task: Todo::default(),
        }
    }

    fn title(&self) -> String {
        String::from("TodoList")
    }

    fn update(&mut self, message: TodoMessage) {
        match message {
            TodoMessage::AddTodo(x) => {
                self.todo_list.push(x);
                self.current_task = Todo::default();
            }
            TodoMessage::DeleteTodo(i) => {
                self.todo_list.swap_remove(i);
            }
            TodoMessage::CurrentTodoMessage(input) => self.current_task.message = input,
            TodoMessage::CurrentTodoTimer(input) => self.current_task.time = input,
        }
    }

    fn view(&self) -> Element<TodoMessage> {
        let tasks =
            self.todo_list
                .iter()
                .enumerate()
                .fold(column().spacing(20), |col, (i, todo)| {
                    let task = text(todo.message.clone());
                    let delete_todo =
                        secondary_button("Delete").on_press(TodoMessage::DeleteTodo(i));
                    col.push(
                        row()
                            .align_items(iced::Alignment::Center)
                            .spacing(10)
                            .push(task)
                            .push(delete_todo),
                    )
                    .push(progress_bar(0.0..=60.0, todo.time.into()))
                });

        let input = text_input(
            "Write your task here",
            &self.current_task.message,
            TodoMessage::CurrentTodoMessage,
        )
        .style(style::Input::Default)
        .padding(10)
        .on_submit(TodoMessage::AddTodo(self.current_task.clone()));

        let set_timer = primary_button(format!("{} hr", self.current_task.time))
            .on_press(TodoMessage::CurrentTodoTimer(self.current_task.time + 1.00));

        let add_todo =
            primary_button("Add Task").on_press(TodoMessage::AddTodo(self.current_task.clone()));

        let content = column()
            .padding(20)
            .align_items(iced::Alignment::Start)
            .push(container(tasks).padding(30))
            .push(
                container(
                    row()
                        .push(
                            container(input)
                                .width(iced::Length::Fill)
                                .center_y()
                                .align_y(iced::alignment::Vertical::Top),
                        )
                        .push(set_timer)
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

fn secondary_button<'a, Message: 'a>(label: &str) -> Button<'a, Message> {
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
        fn active(&self) -> text_input::Style {
            text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border_width: 0.5,
                border_color: Color::from_rgba8(0xEE, 0xEE, 0xEE, 0.5),
                border_radius: 6.0,
            }
        }

        fn focused(&self) -> text_input::Style {
            text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border_width: 0.5,
                border_color: Color::from_rgba8(0xEE, 0xEE, 0xEE, 0.5),
                border_radius: 6.0,
                ..text_input::Style::default()
            }
        }

        fn hovered(&self) -> text_input::Style {
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
