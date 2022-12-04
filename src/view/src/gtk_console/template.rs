use gtk::prelude::*;
use gtk::subclass::prelude::*;

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

impl ObjectImpl for GtkConsoleTemplate {}
impl WidgetImpl for GtkConsoleTemplate {}
impl TextViewImpl for GtkConsoleTemplate {}