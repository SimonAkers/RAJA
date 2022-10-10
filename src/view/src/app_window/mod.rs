pub mod imp;

use gtk::glib;
use glib::Object;
use adw::{gio, Application};

use crate::app_window::imp::AppWindow;

glib::wrapper! {
    pub struct AdwGUI(ObjectSubclass<imp::AppWindow>)
        @extends adw::ApplicationWindow, adw::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl AppWindow {
    pub fn new(app: &Application) -> Self {
        gio::resources_register_include!("app_window.gresource")
            .expect("Failed to register resources.");

        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create Window")
    }
}
