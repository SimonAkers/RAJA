mod template;

use std::borrow::Borrow;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{TextBufferExt, TextBufferExtManual, TextViewExt};
use gtk::TextBuffer;
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

    fn print_err(&self, msg: &str) {
        let buffer = self.buffer();

        self.print(msg);

        let start_iter = buffer.iter_at_offset(buffer.char_count() - msg.len() as i32);
        let end_iter = buffer.end_iter();

        let tag = buffer.create_tag(None, &[("foreground", &"#FF3535")]);

        buffer.apply_tag(&tag.unwrap(), &start_iter, &end_iter);
    }

    fn input(&self) -> Option<&str> {
        todo!()
    }

    fn clear(&self) {
        self.set_buffer(Some(&TextBuffer::default()));
    }
}
