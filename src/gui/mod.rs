use iced::widget::{button, column, text};
use iced::{Element, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    GoToHomePage,
    GoToTestPage,
}

#[derive(Debug)]
enum Page {
    Home,
    Test,
}

impl Default for Page {
    fn default() -> Self {
        Self::Home
    }
}

impl Page {
    pub fn view(&self) -> Element<'static, Message> {
        match self {
            Self::Home => text!("Home page content.").into(),
            Self::Test => text!("Test page content.").into(),
        }
    }
}

#[derive(Default)]
pub struct App {
    page: Page,
}

impl App {
    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::GoToHomePage => self.page = Page::Home,
            Message::GoToTestPage => self.page = Page::Test,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            self.page.view(),
            button("Go home").on_press(Message::GoToHomePage),
            button("Go test").on_press(Message::GoToTestPage),
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
