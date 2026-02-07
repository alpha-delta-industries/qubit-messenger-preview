use iced::widget::{button, column, text};
use iced::{Element, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
pub struct App {
    counter: isize,
}

impl App {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Decrement => self.counter -= 1,
            Message::Increment => self.counter += 1,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text(self.counter.to_string()).size(20),
            button("Increment").on_press(Message::Increment),
            button("Decrement").on_press(Message::Decrement),
        ]
        .spacing(10)
        .into()
    }

    pub fn run() -> iced::Result {
        iced::application(App::new, App::update, App::view)
            .theme(Theme::Dark)
            .run()
    }
}
