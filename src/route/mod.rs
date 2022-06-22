use std::collections::HashMap;
use std::error::Error;
use iced::{Element, Command, Application};

use crate::view;
use crate::view::View;
use crate::view::page::{Page, PageRef};
use crate::message;
use crate::page::Test;

#[derive(Debug, Clone)]
pub enum RouteMessage<'a> {
    Go(String, &'a Routes),
    Back(i32)
}
impl message::MessageHandle for RouteMessage<'_> {
    fn handle(&self, app: &mut crate::App) -> Command<message::Message> {
        match self {
            // go to a route
            Self::Go(path, routes) => {
                app.page = routes.get_page(path);
                Command::none()
            },
            _ => Command::none()
        }
    }
}

pub type Route = (String, PageRef);
pub struct Routes {
    list: HashMap<String, PageRef>,
}
impl Routes {
    pub fn new(pages: Vec<PageRef>) -> Self {
        let mut new = Self {
            list: HashMap::new()
        };
        for page in pages { new.add(page); }
        return new;
    }
    pub fn add(&mut self, p: PageRef) -> &mut Self {
        self.list.insert(p::get_path(), p);
        return self;
    }
    pub fn get_page(&self, r: &str) -> &PageRef {
        match self.list.get(r) {
            Some(b) => b,
            None => self.list.get("404").unwrap()
        }
    }
}