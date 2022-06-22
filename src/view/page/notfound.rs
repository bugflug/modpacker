use iced::{Row, Element, Length, Text};
use crate::message;
use crate::view::View;
use crate::view::page::Page;

#[derive(Debug, Clone)]
pub struct NotFound;
impl View for NotFound {
    fn view(&self) -> Element<message::Message> {
        Row::new().width(Length::Shrink).push(Text::new("404").size(40)).into()
    }
    fn new() -> Self {
        Self{}
    }
}
impl Page for NotFound {
    fn get_path() -> String { "404".to_string() }
}