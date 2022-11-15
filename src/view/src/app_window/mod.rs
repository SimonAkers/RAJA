mod template;

use gtk::{gio, HeaderBar};
use glib::Object;
use glib::subclass::prelude::ObjectSubclassIsExt;
use adw::Application;

glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<template::AppWindowTemplate>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl AppWindow {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder::<AppWindow>()
            .property("application", app)
            .build()
    }

    pub fn header_bar(&self) -> HeaderBar {
        self.imp().header_bar.get()
    }

    pub fn source_view(&self) -> sourceview5::View {
        self.imp().source_view.get()
    }

    pub fn btn_run(&self) -> gtk::Button {
        self.imp().btn_run.get()
    }

    pub fn btn_build(&self) -> gtk::Button {
        self.imp().btn_build.get()
    }
}
