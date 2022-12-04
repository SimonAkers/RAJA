use glib::*;

use crate::gtk_console::GtkConsole;

/**
Ensures that custom widgets are known to GTK. Must be called before an instance
of [`gtk::Application`] is created.
 */
pub fn ensure_types() {
    GtkConsole::ensure_type();
}