mod template;
mod constants;

use std::borrow::{Borrow, BorrowMut};
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{TextBufferExt, TextBufferExtManual, TextViewExt, WidgetExt};

use crate::gtk_console::constants::*;
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
    pub fn start_user_input(&mut self) {
        let mut buffer = self.buffer();
        let (start, end) = buffer.bounds();

        // Mark the beginning of the user's input
        buffer.move_mark_by_name(MARK_START_USER_INPUT, &end);
        // Prevent user from editing pre-existing text
        buffer.apply_tag_by_name(TAG_PROTECTED_TEXT, &start, &end);

        self.set_editable(true);
        self.imp().user_input_started.set(true);

        self.set_cursor_visible(true);
        self.grab_focus();
    }

    pub fn end_user_input(&mut self) {
        let mut buffer = self.buffer();
        let (start, end) = buffer.bounds();

        // Mark the end of the user's input
        buffer.move_mark_by_name(MARK_END_USER_INPUT, &end);
        // Remove tag while user input is disabled
        buffer.remove_tag_by_name(TAG_PROTECTED_TEXT, &start, &end);

        self.set_editable(false);
        self.imp().user_input_started.set(false);

        self.set_cursor_visible(false);
    }

    pub fn user_input_started(&self) -> bool {
        self.imp().user_input_started.get()
    }

    fn print_with_tag(&self, tag: &str, msg: &str) {
        let buffer = self.buffer();

        // Print the message
        self.print(msg);

        // Get the bounds of the message we just printed
        // (this must be done after modifying the buffer)
        let start_iter = buffer.iter_at_offset(buffer.char_count() - msg.len() as i32);
        let end_iter = buffer.end_iter();

        // Apply a tag to style error text (color it)
        buffer.apply_tag_by_name(tag, &start_iter, &end_iter);
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
        self.print("\n");
        self.print_with_tag(TAG_ERROR_TEXT, msg);
    }

    fn print_success(&self, msg: &str) {
        self.print("\n");
        self.print_with_tag(TAG_SUCCESS_TEXT, msg);
    }

    fn input(&self) -> String {
        let buffer = self.buffer();

        // Get the marks surrounding the user input
        let start_mark = buffer.mark(MARK_START_USER_INPUT).unwrap();
        let end_mark = buffer.mark(MARK_END_USER_INPUT).unwrap();

        // Get the iters at those marks
        let start = buffer.iter_at_mark(&start_mark);
        let end = buffer.iter_at_mark(&end_mark);

        // Return the text within those iters
        buffer.text(&start, &end, true).to_string()
    }

    fn clear(&self) {
        let buffer = self.buffer();

        // Remove all tags (e.g., for text color)
        buffer.remove_all_tags(&buffer.start_iter(), &buffer.end_iter());
        // Set the text to empty
        buffer.set_text("");
    }
}
