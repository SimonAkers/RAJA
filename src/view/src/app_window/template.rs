use adw::subclass::prelude::AdwApplicationWindowImpl;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::CompositeTemplate;
use glib::subclass::InitializingObject;

use crate::gtk_console;

/**
The template for [AppWindow][`crate::app_window::AppWindow`] \
which represents an instance of the application window.

This mostly consists of gtk-rs boilerplate and should not be constructed directly.
 */
#[derive(CompositeTemplate, Default)]
#[template(file = "../../res/ui/app_window.ui")]
pub struct AppWindowTemplate {
    #[template_child]
    pub header_bar: TemplateChild<gtk::HeaderBar>,
    #[template_child]
    pub source_view: TemplateChild<sourceview5::View>,
    #[template_child]
    pub console: TemplateChild<gtk_console::GtkConsole>,
    #[template_child]
    pub btn_run: TemplateChild<gtk::Button>,
    #[template_child]
    pub btn_build: TemplateChild<gtk::Button>,
}

/// gtk-rs boilerplate implementation
#[glib::object_subclass]
impl ObjectSubclass for AppWindowTemplate {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "AppWindow";

    type Type = super::AppWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

/// gtk-rs boilerplate implementation
impl ObjectImpl for AppWindowTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for AppWindowTemplate {}
impl WindowImpl for AppWindowTemplate {}
impl ApplicationWindowImpl for AppWindowTemplate {}
impl AdwApplicationWindowImpl for AppWindowTemplate {}