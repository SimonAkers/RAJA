/*!
A simple wrapper for saying Rc::new(RefCell::new(Object)).

Based on https://gist.github.com/stevedonovan/7e3a6d8c8921e3eff16c4b11ab82b8d7
*/

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct Shared<T> {
    rc: Rc<RefCell<T>>
}

impl<T> Shared<T> {
    pub fn new(data: T) -> Self {
        Self { rc: Rc::new(RefCell::new(data)) }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.rc.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.rc.borrow_mut()
    }

    pub fn as_ptr(&self) -> *mut T {
        self.rc.as_ptr()
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self { rc: self.rc.clone() }
    }
}