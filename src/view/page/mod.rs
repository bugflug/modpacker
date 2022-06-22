use iced::Element;
use crate::message::MessageHandle;
use crate::view::View;

// declare pages
pub mod test; pub use test::Test;
pub mod notfound; pub use notfound::NotFound;

pub fn list() -> Vec<Box<dyn Page>> {
    vec![
        Test::boxed(),
        NotFound::boxed()
    ]
}

pub trait Page: View {
    fn get_path() -> String where Self: Sized;
}
pub type PageRef = Box<dyn Page>;