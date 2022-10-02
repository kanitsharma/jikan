use iced::pure::{column, container, row, text_input, Element, Sandbox};
use iced::Settings;
// mod numeric_input;
mod task;
mod styles;
#[path = "components/buttons.rs"] mod buttons;

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
        let tasks = self
            .todo_list
            .iter()
            .enumerate()
            .fold(column().spacing(20), |col, (i, t)| {
                let task = task::Task::new(String::from(t), i).view();
                col.push(row().push(task))
            });

        let input = text_input(
            "Write your task here",
            &self.current_task,
            TodoMessage::CurrentTodo,
        )
        .style(styles::Input::Default)
        .padding(10)
        .on_submit(TodoMessage::AddTodo(self.current_task.clone()));

        let add_todo =
            buttons::primary_button("Add Task").on_press(TodoMessage::AddTodo(self.current_task.clone()));

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
                        .push(add_todo)
                        .align_items(iced::Alignment::Start)
                        .spacing(10),
                )
                .height(iced::Length::Fill)
                .align_y(iced::alignment::Vertical::Bottom),
            );

        container(content)
            .style(styles::Container::Default)
            .center_x()
            .align_y(iced::alignment::Vertical::Bottom)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

