mod template;

use std::borrow::Borrow;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{TextBufferExt, TextBufferExtManual, TextViewExt};
use crate::traits::Console;

glib::wrapper! {
    /**
    A custom widget which represents a graphical console for text I/O.

    # See also:
    - [GtkConsoleTemplate][`crate::gtk_console::template::GtkConsoleTemplate`]
    - [Console][`crate::traits::Console`]
     */
    pub struct GtkConsole(ObjectSubclass<template::GtkConsoleTemplate>)
        @extends gtk::TextView, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

/// See [Console][`crate::traits::Console`] for docs.
impl Console for GtkConsole {
    fn print(&self, msg: &str) {
        // Insert the message at the end of the buffer
        self.buffer().insert(&mut self.buffer().end_iter(), msg);
        // Scroll the widget to the bottom
        self.scroll_to_iter(&mut self.buffer().end_iter(), 0.0, true, 0.0, 0.0);
    }

    fn print_err(&self, msg: &str) {
        let buffer = self.buffer();

        // Print the message
        self.print(msg);

        // Get the bounds of the message we just printed
        // (this must be done after modifying the buffer)
        let start_iter = buffer.iter_at_offset(buffer.char_count() - msg.len() as i32);
        let end_iter = buffer.end_iter();

        // Create a tag with a foreground color
        let tag = buffer.create_tag(None, &[("foreground", &"#FF3535")]);
        // Apply the tag to color the message
        buffer.apply_tag(&tag.unwrap(), &start_iter, &end_iter);
    }

    fn input(&self) -> Option<&str> {
        todo!()
    }

    fn clear(&self) {
        let buffer = self.buffer();

        // Remove all tags (e.g., for text color)
        buffer.remove_all_tags(&buffer.start_iter(), &buffer.end_iter());
        // Set the text to empty
        buffer.set_text("");
    }
}
