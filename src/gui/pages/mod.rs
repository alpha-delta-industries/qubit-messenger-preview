use crate::gui::{
    Page,
    pages::{home::Home, page_not_found::PageNotFound, test::Test},
};

macro_rules! page {
    ($struct_name:ident, $body:block) => {
        pub struct $struct_name {}

        impl $struct_name {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl crate::gui::Page for $struct_name {
            fn view(&self) -> iced::Element<'static, crate::gui::Message> $body
        }
    };
}

pub mod home;
pub mod page_not_found;
pub mod test;

pub fn get_page_by_str(page_name: &str) -> Box<dyn Page> {
    match page_name {
        "home" => Box::new(Home::new()),
        "test" => Box::new(Test::new()),
        _ => Box::new(PageNotFound::new()),
    }
}
