use adw::subclass::prelude::AdwApplicationWindowImpl;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::CompositeTemplate;
use glib::subclass::InitializingObject;

use crate::gtk_console;

/**
The template for [MainView][`crate::main_view::MainView`] \
which represents the main (center) view of the application.

This mostly consists of gtk-rs boilerplate and should not be constructed directly.
 */
#[derive(CompositeTemplate, Default)]
#[template(file = "template.ui")]
pub struct MainViewTemplate {
    #[template_child]
    pub source_view: TemplateChild<sourceview5::View>,
    #[template_child]
    pub console: TemplateChild<gtk_console::GtkConsole>,
}

/// gtk-rs boilerplate implementation
#[glib::object_subclass]
impl ObjectSubclass for MainViewTemplate {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MainView";

    type Type = super::MainView;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

/// gtk-rs boilerplate implementation
impl ObjectImpl for MainViewTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for MainViewTemplate {}
impl WindowImpl for MainViewTemplate {}
impl BoxImpl for MainViewTemplate {}