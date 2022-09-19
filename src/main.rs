// use std::process::Command;

mod steps;

use iced::pure::{button, column, container, row, text, Element, Sandbox};
use iced::Settings;

fn main() -> Result<(), iced::Error> {
    Counter::run(Settings::default())
}

#[derive(Clone)]
struct Counter {
    todo_list: Vec<String>,
}

#[derive(Debug, Clone)]
enum TodoMessage {
    AddTodo(String),
    DeleteTodo(usize),
}

impl Sandbox for Counter {
    type Message = TodoMessage;

    fn new() -> Self {
        Counter {
            todo_list: vec![String::from("Dummy Task")],
        }
    }

    fn title(&self) -> String {
        String::from("TodoList")
    }

    fn update(&mut self, message: TodoMessage) {
        match message {
            TodoMessage::AddTodo(x) => self.todo_list.push(x),
            TodoMessage::DeleteTodo(i) => {
                self.todo_list.swap_remove(i);
                return;
            }
        }
    }

    fn view(&self) -> Element<TodoMessage> {
        let tasks =
            self.todo_list
                .iter()
                .enumerate()
                .fold(column().spacing(20), move |col, (i, task)| {
                    let task = text(task);
                    let delete_todo = button("Remove").on_press(TodoMessage::DeleteTodo(i));
                    col.push(row().spacing(10).push(task).push(delete_todo))
                });

        let add_todo = button("Add Todo").on_press(TodoMessage::AddTodo(String::from("New Task")));

        let content = column()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(tasks)
            .push(add_todo);

        container(content)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
