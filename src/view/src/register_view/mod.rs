mod template;

use std::borrow::{Borrow, BorrowMut};
use glib::Object;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::{ListBox, ListBoxRow, Orientation, ScrolledWindow, SelectionMode};
use gtk::prelude::*;

use gtk::prelude::BoxExt;
use model::RegisterFile;

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
    pub fn new() -> Self {

        let list = ListBox::new();
        list.set_hexpand(true);
        list.set_vexpand(true);
        list.set_selection_mode(SelectionMode::None);

        for i in 1..100 {
            list.append(&gtk::Label::new(Some(&format!("Row {i}"))));
        }

        let obj: RegisterView = Object::builder().build();
        obj.set_hexpand(true);
        obj.set_vexpand(true);

        obj.append(
        &ScrolledWindow::builder()
            .height_request(300)
            .width_request(200)
            .child(&list)
            .build()
        );

        obj
    }
}
