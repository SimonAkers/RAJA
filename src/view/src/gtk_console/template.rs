use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct GtkConsoleTemplate;

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