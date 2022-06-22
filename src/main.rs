use std::collections::HashMap;
use iced::{Application, Command, Column, Element, executor, Length, Settings};

mod message;
use message::{Message, MessageHandle};
mod view;
use view::{page, View, ViewMessage};
mod route;

// main app controller
struct Flags {
    title: String,
    routes: &'static route::Routes
}
struct App<'app> {
    title: String,
    route: String,
    page: &'app Box<dyn page::Page>
}
impl<'app> Application for App<'app> {
    type Executor = executor::Default;
    type Message = Message<'app>;
    type Flags = Flags;

    // init
    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {
            title: flags.title,
            route: "404".to_string(),
            page: flags.routes.get_page("404")
        }, Command::none())
    }

    // app title
    fn title(&self) -> String { "modpacker".to_string() }

    // updates
    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        match msg {
            Message::Route(m) => m.handle(self),
            Message::View(m) => m.handle(self),
            Message::None | _ => Command::none()
        }
    }

    // dynamically match the correct page with the current route
    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(self.page.view())
            .into()
    }
}

fn main() -> iced::Result {
    // setup different modules
    let routes = route::Routes::new(page::list());

    // launch the fuckin thing already
    App::run(Settings::with_flags(Flags {
        title: "modpacker".to_string(),
        routes: &routes
    }))
}