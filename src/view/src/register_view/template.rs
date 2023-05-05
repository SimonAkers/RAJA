use std::cell::{Cell, RefCell};
use glib::subclass::InitializingObject;
use gtk::{CompositeTemplate, Grid};
use gtk::pango::{AttrFontDesc, Attribute, AttrList, FontDescription};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use util::settings::Settings;

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

    // An AttrList containing font info
    pub font_attrs: RefCell<AttrList>,
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

        // Load the app settings
        let settings = Settings::load();

        // Create an AttrList of the font
        let attrs = AttrList::new();
        attrs.insert(Attribute::from(
            AttrFontDesc::new(
                &FontDescription::from_string(settings.mono_font())
            )
        ));

        // Store the font
        self.font_attrs.replace(attrs);
    }
}

impl WidgetImpl for RegisterViewTemplate {}
impl BoxImpl for RegisterViewTemplate {}