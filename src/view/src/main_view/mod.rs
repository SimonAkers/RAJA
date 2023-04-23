mod template;

use gtk::{gio, HeaderBar};
use glib::Object;
use glib::subclass::prelude::ObjectSubclassIsExt;
use adw::Application;
use crate::gtk_console::GtkConsole;
use crate::widget;

glib::wrapper! {
    /**
    A "subclass" of [`gtk::ApplicationWindow`].
    See also: [AppWindowTemplate][`crate::app_window::template::AppWindowTemplate`]
     */
    pub struct MainView(ObjectSubclass<template::MainViewTemplate>)
        @extends gtk::Box, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainView {
    /**
    Creates a new AppWindow.

    # Arguments
    - `app` - The application that the AppWindow is associated with.
     */
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder::<MainView>()
            .property("application", app)
            .build()
    }

    widget!(source_view, sourceview5::View);
    widget!(console, GtkConsole);
}
