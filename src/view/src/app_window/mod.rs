mod template;

use gtk::{gio, HeaderBar};
use glib::Object;
use glib::subclass::prelude::ObjectSubclassIsExt;
use adw::Application;
use crate::gtk_console::GtkConsole;

glib::wrapper! {
    /**
    A "subclass" of [`gtk::ApplicationWindow`].
    See also: [AppWindowTemplate][`crate::app_window::template::AppWindowTemplate`]
     */
    pub struct AppWindow(ObjectSubclass<template::AppWindowTemplate>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

/// Macro for generating widget convenience getter
macro_rules! widget {
    ($widget:ident, $t:ty) => {
        #[inline(always)]
        pub fn $widget(&self) -> $t {
            self.imp().$widget.get()
        }
    }
}

impl AppWindow {
    /**
    Creates a new AppWindow.

    # Arguments
    - `app` - The application that the AppWindow is associated with.
     */
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder::<AppWindow>()
            .property("application", app)
            .build()
    }

    // TODO: Further abstract these macro calls into a derive macro
    widget!(header_bar, HeaderBar);
    widget!(source_view, sourceview5::View);
    widget!(console, GtkConsole);
    widget!(btn_run, gtk::Button);
    widget!(btn_build, gtk::Button);
    widget!(btn_settings, gtk::Button);
}
