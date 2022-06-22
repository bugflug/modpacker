use std::fmt::Error;
use iced::{Element, Command};
use crate::message;

pub mod page;

#[derive(Debug, Clone)]
pub enum ViewMessage {
    None
}
impl message::MessageHandle for ViewMessage {
    fn handle(&self, app: &mut crate::App) -> Command<message::Message> {
        match self {
            _ => Command::none()
        }
    }
}

// view component
pub trait View {
    fn view(&self) -> Element<message::Message>;
    fn new() -> Self where Self: Sized; // Sized is fine here, traits shouldn't implement it
    fn boxed() -> Box<Self> where Self: Sized { Box::new(Self::new()) }
}