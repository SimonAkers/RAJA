mod template;

use glib::subclass::prelude::ObjectSubclassIsExt;

glib::wrapper! {
    pub struct GtkConsole(ObjectSubclass<template::GtkConsoleTemplate>)
        @extends gtk::TextView, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}
