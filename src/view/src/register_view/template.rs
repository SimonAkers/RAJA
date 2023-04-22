use std::borrow::BorrowMut;
use std::cell::Cell;

use gtk::prelude::*;
use gtk::subclass::prelude::*;

/**
The template for [RegisterView][`crate::register_view::RegisterView`] \
which represents a console widget.

This mostly consists of gtk-rs boilerplate and should not be constructed directly.
 */
#[derive(Default)]
pub struct RegisterViewTemplate {

}

/// gtk-rs boilerplate implementation
#[glib::object_subclass]
impl ObjectSubclass for RegisterViewTemplate {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "RegisterView";
    type Type = super::RegisterView;
    type ParentType = gtk::Box;
}

impl RegisterViewTemplate {

}

impl ObjectImpl for RegisterViewTemplate {}
impl WidgetImpl for RegisterViewTemplate {}
impl BoxImpl for RegisterViewTemplate {}