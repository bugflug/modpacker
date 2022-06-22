use iced::{Column, Element, Length, Text};
use crate::message;
use crate::view::View;
use crate::view::page::Page;

#[derive(Debug, Clone)]
pub struct Test;
impl View for Test {
    fn view(&self) -> Element<message::Message> {
        Column::new().width(Length::Shrink).push(Text::new("twerk").size(40)).into()
    }
    fn new() -> Self {
        Self{}
    }
}
impl Page for Test {
    fn get_path() -> String { "test".to_string() }
}