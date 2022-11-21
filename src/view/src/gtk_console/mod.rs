mod template;

use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{TextBufferExt, TextViewExt};
use crate::traits::Console;

glib::wrapper! {
    pub struct GtkConsole(ObjectSubclass<template::GtkConsoleTemplate>)
        @extends gtk::TextView, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Console for GtkConsole {
    fn print(&self, msg: &str) {
        // Insert the message at the end of the buffer
        self.buffer().insert(&mut self.buffer().end_iter(), msg);
        self.scroll_to_iter(&mut self.buffer().end_iter(), 0.0, true, 0.0, 0.0);
    }

    fn input(&self) -> Option<&str> {
        todo!()
    }

    fn clear(&self) {
        self.buffer().set_text("");
    }
}
