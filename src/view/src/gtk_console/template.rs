use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::CompositeTemplate;
use glib::subclass::InitializingObject;

#[derive(CompositeTemplate, Default)]
#[template(file = "../../res/ui/gtk_console.ui")]
pub struct GtkConsoleTemplate {

}

#[glib::object_subclass]
impl ObjectSubclass for GtkConsoleTemplate {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "GtkConsole";

    type Type = super::GtkConsole;
    type ParentType = gtk::TextView;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for GtkConsoleTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for GtkConsoleTemplate {}
impl TextViewImpl for GtkConsoleTemplate {}