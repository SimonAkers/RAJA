/// A trait defining required functionality for the application's GUI.
pub trait AppUI {
    fn run(&self);
    fn get_source(&self) -> Box<dyn Source>;
}

pub trait Source {
    fn get_text(&self) -> String;
    fn clear(&self);
}