mod template;

use std::borrow::{Borrow, BorrowMut};
use glib::Object;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::{ListBox, ListBoxRow, Orientation, ScrolledWindow, SelectionMode};
use gtk::prelude::*;

use gtk::prelude::BoxExt;
use model::RegisterFile;
use crate::widget;

glib::wrapper! {
    /**
    A custom widget which represents a graphical console for text I/O.

    # See also:
    - [GtkConsoleTemplate][`crate::gtk_console::template::GtkConsoleTemplate`]
    - [Console][`crate::traits::Console`]
     */
    pub struct RegisterView(ObjectSubclass<template::RegisterViewTemplate>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl RegisterView {
    widget!(list_box, ListBox);
}
