use iced::pure::{column, container, progress_bar, row, text_input, Element, Sandbox};
use iced::Settings;
// mod numeric_input;
#[path = "components/buttons.rs"]
mod buttons;
mod styles;
#[path = "components/task.rs"]
mod task;

fn main() -> Result<(), iced::Error> {
    App::run(Settings::default())
}

#[derive(Clone, Default)]
struct App {
    todo_list: Vec<Todo>,
    current_task: Todo,
}

#[derive(Debug, Clone, Default)]
pub struct Todo {
    message: String,
    time: f32,
}

#[derive(Debug, Clone)]
pub enum TodoMessage {
    AddTodo(Todo),
    DeleteTodo(usize),
    CurrentTodoMessage(String),
    CurrentTodoTimer(f32),
}

impl Sandbox for App {
    type Message = TodoMessage;

    fn new() -> Self {
        App {
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
        let tasks = self
            .todo_list
            .iter()
            .enumerate()
            .fold(column().spacing(20), |col, (i, t)| {
                let task = task::Task::new(t.clone().message, i).view();
                col.push(row().push(task).push(progress_bar(0.0..=60.0, t.time)))
            });

        let input = text_input(
            "Write your task here",
            &*self.current_task.message,
            TodoMessage::CurrentTodoMessage,
        )
        .style(styles::Input::Default)
        .padding(10)
        .on_submit(TodoMessage::AddTodo(self.current_task.clone()));

        let set_timer = buttons::primary_button(&format!("{} hr", self.current_task.time))
            .on_press(TodoMessage::CurrentTodoTimer(
                &self.current_task.time + 1.00,
            ));

        let add_todo = buttons::primary_button("Add Task")
            .on_press(TodoMessage::AddTodo(self.current_task.clone()));

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
            .style(styles::Container::Default)
            .center_x()
            .align_y(iced::alignment::Vertical::Bottom)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
