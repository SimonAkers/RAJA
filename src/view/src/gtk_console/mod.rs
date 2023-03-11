mod template;
mod buffer_tags;

use std::borrow::Borrow;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{TextBufferExt, TextBufferExtManual, TextViewExt};

use crate::gtk_console::buffer_tags::*;
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

impl GtkConsole {
    /**
    Enables or disables user input at the end of the console.

    # Arguments
    - `allow` - Whether to enable (true) or disable (false) user input.
    */
    pub fn allow_user_input(&mut self, allow: bool) {
        // Get a reference to the buffer and its bounds
        let mut buffer = self.buffer();
        let start = buffer.start_iter();
        let end = buffer.end_iter();

        // Set the TextView to be editable or not
        self.set_editable(allow);

        if allow {
            // Prevent user from editing pre-existing text
            buffer.apply_tag_by_name(TAG_PROTECTED_TEXT, &start, &end);
        } else {
            // Remove the tag while user input is disabled to avoid possible interferences
            // (this may be unnecessary)
            buffer.remove_tag_by_name(TAG_PROTECTED_TEXT, &start, &end);
        }
    }
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

        // Apply a tag to style error text (color it)
        buffer.apply_tag_by_name(TAG_ERROR_TEXT, &start_iter, &end_iter);
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
