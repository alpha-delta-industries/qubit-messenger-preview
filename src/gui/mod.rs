use iced::widget::{button, column};
use iced::{Element, Theme};

use crate::gui::pages::get_page_by_str;

mod pages;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Message {
    GoToPage(String),
}

trait Page {
    fn view(&self) -> Element<'static, Message>;
}

pub struct App {
    page: Box<dyn Page>,
}

impl App {
    fn new() -> Self {
        Self {
            page: Box::new(pages::home::Home::new()),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::GoToPage(page_name) => self.page = get_page_by_str(page_name.as_str()),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            self.page.view(),
            button("Go home").on_press(Message::GoToPage("home".to_string())),
            button("Go test").on_press(Message::GoToPage("test".to_string())),
            button("Go 404").on_press(Message::GoToPage("404".to_string())),
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
