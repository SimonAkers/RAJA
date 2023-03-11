// Based on concepts from https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust

#[derive(Default)]
pub struct Callback {
    callback: Option<Box<dyn FnMut(Option<&String>)>>
}

impl Callback {
    pub fn new(callback: Box<dyn FnMut(Option<&String>)>) -> Self {
        Self { callback: Some(callback) }
    }

    pub fn call(&mut self, info: Option<&String>) {
        match &mut self.callback {
            Some(callback) => callback(info),
            None => (),
        }
    }
}