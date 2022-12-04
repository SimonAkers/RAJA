mod template;

use gtk::{gio, HeaderBar};
use glib::Object;
use glib::subclass::prelude::ObjectSubclassIsExt;
use adw::Application;
use crate::gtk_console;

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

    /// Returns the window's header bar.
    pub fn header_bar(&self) -> HeaderBar {
        self.imp().header_bar.get()
    }

    /// Returns the window's source view.
    pub fn source_view(&self) -> sourceview5::View {
        self.imp().source_view.get()
    }

    /// Returns the window's console.
    pub fn console(&self) -> gtk_console::GtkConsole {
        self.imp().console.get()
    }

    /// Returns the window's run button.
    pub fn btn_run(&self) -> gtk::Button {
        self.imp().btn_run.get()
    }

    /// Returns the window's build button.
    pub fn btn_build(&self) -> gtk::Button {
        self.imp().btn_build.get()
    }
}
