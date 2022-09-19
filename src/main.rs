// use std::process::Command;

mod steps;

use std::clone;

use iced::widget::{button, Button, Column, Container, Text};
use iced::{alignment, Length, Sandbox};
use iced::{Element, Renderer, Row, Settings};

fn main() -> Result<(), iced::Error> {
    Counter::run(Settings::default())
}

#[derive(Clone)]
struct Counter {
    todo_list: Vec<String>,
    add_todo_button: button::State,
    delete_todo_button: button::State,
}

#[derive(Debug, Clone)]
enum TodoMessage {
    AddTodo(String),
    DeleteTodo(usize),
}

fn button<'a, Message: Clone>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(alignment::Horizontal::Center),
    )
    .padding(12)
    .width(Length::Units(100))
}

impl Sandbox for Counter {
    type Message = TodoMessage;

    fn new() -> Self {
        Counter {
            todo_list: vec![String::from("Dummy Task")],
            add_todo_button: button::State::new(),
            delete_todo_button: button::State::new(),
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

    fn view(&mut self) -> Element<TodoMessage> {
        let tasks =
            self.todo_list
                .iter()
                .enumerate()
                .fold(Row::new().spacing(20),|row, (i, task)| {
        
                    let task = Text::new(task);
                    
                    let delete_todo = button(&mut self.delete_todo_button, "Remove").on_press(TodoMessage::DeleteTodo(i));
                    row.push(task).push(delete_todo)
                });

        let add_todo = button(&mut self.add_todo_button, "Add Todo")
            .on_press(TodoMessage::AddTodo(String::from("New Task")));
        // Container::new(col)
        //     .center_x()
        //     .center_y()
        //     .width(iced::Length::Fill)
        //     .height(iced::Length::Fill).into()
        Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(tasks)
            .push(add_todo)
            .into()
    }
}
