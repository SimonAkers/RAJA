/// A trait defining required functionality for the application's UI.
pub trait AppUI {
    fn start(&mut self);
    fn get_source(&self) -> Box<dyn Source>;
    fn get_console(&self) -> Box<dyn Console>;
}

pub trait Source {
    fn get_text(&self) -> String;
    fn clear(&self);
}

pub trait Console {
    fn print(&self, msg: &str);
    fn input(&self) -> Option<&str>;
    fn clear(&self);
}