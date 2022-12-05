use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

/**
A simple wrapper for saying Rc::new(RefCell::new(Object)).

Based on https://gist.github.com/stevedonovan/7e3a6d8c8921e3eff16c4b11ab82b8d7
 */
pub struct Shared<T> {
    rc: Rc<RefCell<T>>
}

impl<T> Shared<T> {
    /**
    Returns a new Shared instance containing a given object.

    # Arguments
    - `object` - The object to wrap.
     */
    pub fn new(object: T) -> Self {
        Self { rc: Rc::new(RefCell::new(object)) }
    }

    /// Returns a borrowed reference to the shared object.
    pub fn borrow(&self) -> Ref<T> {
        self.rc.borrow()
    }

    /// Returns a borrowed mutable reference to the shared object.
    pub fn borrow_mut(&self) -> RefMut<T> {
        self.rc.borrow_mut()
    }

    /// Returns a mutable pointer to the shared object.
    pub fn as_ptr(&self) -> *mut T {
        self.rc.as_ptr()
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self { rc: self.rc.clone() }
    }
}