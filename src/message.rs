use iced::Command;

use crate::view::ViewMessage;
use crate::route::RouteMessage;

#[derive(Debug, Clone)]
pub enum Message<'a> {
    View(ViewMessage),
    Route(RouteMessage<'a>),
    None
}

pub trait MessageHandle {
    fn handle(&self, app: &mut crate::App) -> Command<Message>;
}