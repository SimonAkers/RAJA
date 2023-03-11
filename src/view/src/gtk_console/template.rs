use std::borrow::BorrowMut;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::gtk_console::buffer_tags::*;

/**
The template for [GtkConsole][`crate::gtk_console::GtkConsole`] \
which represents a console widget.

This mostly consists of gtk-rs boilerplate and should not be constructed directly.
 */
#[derive(Default)]
pub struct GtkConsoleTemplate;

/// gtk-rs boilerplate implementation
#[glib::object_subclass]
impl ObjectSubclass for GtkConsoleTemplate {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "GtkConsole";
    type Type = super::GtkConsole;
    type ParentType = gtk::TextView;
}

impl ObjectImpl for GtkConsoleTemplate {
    fn constructed(&self) {
        self.parent_constructed();

        // Get a reference to the console's text buffer
        let mut buffer = self.obj().borrow_mut().buffer();

        // Create a tag used to prevent the user from editing certain text
        buffer.create_tag(Some(TAG_PROTECTED_TEXT), &[("editable", &false)]);

        // Create a tag used to highlight error text in red
        buffer.create_tag(Some(TAG_ERROR_TEXT), &[("foreground", &"#FF3535")]);
    }
}

impl WidgetImpl for GtkConsoleTemplate {}
impl TextViewImpl for GtkConsoleTemplate {}