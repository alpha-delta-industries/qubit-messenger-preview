use iced::alignment::Vertical;
use iced::widget::{Button, Space, Svg, Text, button, column, container, row, text};
use iced::{Element, Length, Theme};

macro_rules! button_with_icon {
    ($icon_path:literal, $button_text:literal) => {
        Button::new(
            row![
                Svg::from_path($icon_path).height(25).width(25),
                Space::new().width(10),
                Text::new($button_text),
            ]
            .align_y(Vertical::Center),
        )
    };
}

macro_rules! left_nav_button {
    ($icon_path:literal, $button_text:literal, $message:expr) => {
        button_with_icon!($icon_path, $button_text)
            .on_press($message)
            .width(175)
            .style(button::secondary)
    };
}

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
            Self::Test => Svg::from_path("./assets/icons/activity.svg")
                .height(100)
                .width(100)
                .into(),
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
        container(
            row![
                container(
                    column![
                        left_nav_button!(
                            "assets/icons/activity.svg",
                            "Home",
                            Message::GoToHomePage
                        ),
                        left_nav_button!(
                            "assets/icons/activity.svg",
                            "Test",
                            Message::GoToTestPage
                        )
                    ]
                    .spacing(2)
                )
                .width(200)
                .align_y(Vertical::Center),
                container(self.page.view()).width(Length::Fill)
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(Vertical::Center)
        .into()
    }

    pub fn run() -> iced::Result {
        iced::application(App::new, App::update, App::view)
            .theme(Theme::Dark)
            .title("Untitled P2P messenger")
            .window_size((500, 400))
            .run()
    }
}
