use std::borrow::BorrowMut;
use std::cell::Cell;

use gtk::{EventControllerFocus, GestureClick};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::gtk_console::constants::*;

/**
The template for [GtkConsole][`crate::gtk_console::GtkConsole`] \
which represents a console widget.

This mostly consists of gtk-rs boilerplate and should not be constructed directly.
 */
#[derive(Default)]
pub struct GtkConsoleTemplate {
    pub user_input_started: Cell<bool>
}

/// gtk-rs boilerplate implementation
#[glib::object_subclass]
impl ObjectSubclass for GtkConsoleTemplate {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "GtkConsole";
    type Type = super::GtkConsole;
    type ParentType = gtk::TextView;
}

impl GtkConsoleTemplate {
    fn connect_click_handler(&self) {
        let controller = GestureClick::new();

        let console = self.obj().clone();
        controller.connect_pressed(move |_, _, _, _| {
            console.set_cursor_visible(true);
        });

        self.obj().add_controller(controller);
    }

    fn connect_focus_handler(&self) {
        let controller = EventControllerFocus::new();

        let console = self.obj().clone();
        controller.connect_enter(move |_| {
            console.set_cursor_visible(true);
        });

        self.obj().add_controller(controller);
    }
}

impl ObjectImpl for GtkConsoleTemplate {
    fn constructed(&self) {
        self.parent_constructed();

        // Get a reference to the console's text buffer
        let mut buffer = self.obj().borrow_mut().buffer();

        // Create a tag used to prevent the user from editing certain text
        buffer.create_tag(Some(TAG_PROTECTED_TEXT), &[("editable", &false)]);

        // Create a tag used to highlight error text in red
        buffer.create_tag(Some(TAG_ERROR_TEXT), &[("foreground", &"#FF3535")]);

        // Create a tag used to highlight success text
        buffer.create_tag(Some(TAG_SUCCESS_TEXT), &[("foreground", &"#6EDB71")]);

        // Create a mark used to determine the beginning of user input
        buffer.create_mark(Some(MARK_START_USER_INPUT), &buffer.start_iter(), true);

        // Create a mark used to determine the end of user input
        buffer.create_mark(Some(MARK_END_USER_INPUT), &buffer.start_iter(), true);

        // Connect handlers to ensure that the cursor is visible when needed
        self.connect_click_handler();
        self.connect_focus_handler();
    }
}

impl WidgetImpl for GtkConsoleTemplate {}
impl TextViewImpl for GtkConsoleTemplate {}