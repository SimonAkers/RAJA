mod template;

use std::borrow::{Borrow, BorrowMut};
use glib::Object;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::{Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, SelectionMode};
use gtk::prelude::*;

use model::RegisterFile;
use crate::widget;

glib::wrapper! {
    /**
    A custom widget which represents a view of register data.

    # See also:
    - [RegisterView][`crate::register_view::template::RegisterView`]
     */
    pub struct RegisterView(ObjectSubclass<template::RegisterViewTemplate>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl RegisterView {
    pub fn update(&mut self, reg_file: &RegisterFile<u32>) {
        let mut listbox = self.list_box();

        // Clear the listbox
        while let Some(row) = listbox.last_child() {
            listbox.remove(&row);
        }

        // Populate the listbox
        for (name, value) in reg_file.map().iter() {
            let text = format!("{name}: {value:#08x}");
            listbox.append(&Label::new(Some(text.as_str())))
        }
    }

    widget!(list_box, ListBox);
}
