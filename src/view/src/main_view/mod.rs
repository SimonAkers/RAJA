mod template;

use gtk::gio;
use glib::subclass::prelude::ObjectSubclassIsExt;
use crate::gtk_console::GtkConsole;
use crate::widget;

glib::wrapper! {
    pub struct MainView(ObjectSubclass<template::MainViewTemplate>)
        @extends gtk::Box, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainView {
    widget!(source_view, sourceview5::View);
    widget!(console, GtkConsole);
}
