use std::cell::RefCell;
use std::rc::Rc;

pub struct RcMut<T> {
    rc: Rc<RefCell<T>>
}

impl<T> RcMut<T> {
    pub fn new(data: T) -> Self {
        Self { rc: Rc::new(RefCell::new(data)) }
    }
}