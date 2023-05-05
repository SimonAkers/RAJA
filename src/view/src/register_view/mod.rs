mod template;

use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
use glib::{BoolError, Object, Value};
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::{Align, Grid, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, SelectionMode};
use gtk::pango::{AttrFontDesc, Attribute, AttrList, AttrString, FontDescription};
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
        let mut grid = self.grid();

        // Clear the grid
        while let Some(row) = grid.last_child() {
            grid.remove(&row);
        }

        let attrs = self.imp().font_attrs.borrow();

        // Populate the grid
        for (i, (name, value)) in reg_file.map().iter().enumerate() {
            let name = Label::builder()
                .label(name)
                .halign(Align::End)
                .attributes(&attrs)
                .build();

            let value = Label::builder()
                .label(format!("{value:#010x}"))
                .halign(Align::Start)
                .attributes(&attrs)
                .build();

            grid.attach(&name, 0, i as i32, 1, 1);
            grid.attach(&value, 1, i as i32, 1, 1);
        }
    }

    widget!(grid, Grid);
}
