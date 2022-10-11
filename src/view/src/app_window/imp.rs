use adw::subclass::prelude::AdwApplicationWindowImpl;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::glib;
use gtk::CompositeTemplate;
use glib::subclass::InitializingObject;

#[derive(CompositeTemplate, Default)]
#[template(file = "../../res/ui/main.ui")]
pub struct AppWindow {
    #[template_child]
    pub header_bar: TemplateChild<gtk::HeaderBar>,
    #[template_child]
    pub source_view: TemplateChild<sourceview5::View>,
}

#[glib::object_subclass]
impl ObjectSubclass for AppWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "RajaAppWindow";

    type Type = super::AppWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for AppWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for AppWindow {}
impl WindowImpl for AppWindow {}
impl ApplicationWindowImpl for AppWindow {}
impl AdwApplicationWindowImpl for AppWindow {}