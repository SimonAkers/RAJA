mod template;

use gtk::{gio, HeaderBar};
use glib::Object;
use glib::subclass::prelude::ObjectSubclassIsExt;
use adw::Application;
use crate::gtk_console::GtkConsole;
use crate::main_view::MainView;
use crate::register_view::RegisterView;
use crate::widget;

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

    // TODO: Further abstract these macro calls into a derive macro
    widget!(header_bar, HeaderBar);
    widget!(btn_run, gtk::Button);
    widget!(btn_build, gtk::Button);
    widget!(btn_settings, gtk::Button);
    widget!(main_view, MainView);
    widget!(register_view, RegisterView);
}