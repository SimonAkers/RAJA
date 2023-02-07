#[derive(Default)]
pub struct Callback<T> {
    callback: Option<Box<dyn FnMut(&T)>>
}

impl<T> Callback<T> {
    pub fn new(callback: Box<dyn FnMut(&T)>) -> Self {
        Self { callback: Some(callback) }
    }

    pub fn call(&mut self, t: &T) {
        match &mut self.callback {
            Some(callback) => callback(t),
            None => (),
        }
    }
}