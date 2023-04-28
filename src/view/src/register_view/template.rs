use glib::subclass::InitializingObject;
use gtk::{CompositeTemplate, Grid};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

/**
The template for [RegisterView][`crate::register_view::RegisterView`] \
which is a widget for viewing register data.

This mostly consists of gtk-rs boilerplate and should not be constructed directly.
 */
#[derive(CompositeTemplate, Default)]
#[template(file = "template.ui")]
pub struct RegisterViewTemplate {
    #[template_child]
    pub grid: TemplateChild<Grid>,
}

/// gtk-rs boilerplate implementation
#[glib::object_subclass]
impl ObjectSubclass for RegisterViewTemplate {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "RegisterView";
    type Type = super::RegisterView;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for RegisterViewTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for RegisterViewTemplate {}
impl BoxImpl for RegisterViewTemplate {}