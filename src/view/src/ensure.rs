use glib::*;

use crate::gtk_console::GtkConsole;

pub fn ensure_types() {
    GtkConsole::ensure_type();
}