use crate::{
    parser::model::{DATA_BASE, STACK_BASE, TEXT_BASE},
    Machine,
};

pub struct MemoryView<'a> {
    machine: &'a mut Machine,
    view_address: &'a mut usize,
    view_endian: &'a mut bool,
}

impl<'a> MemoryView<'a> {
    pub fn new(
        machine: &'a mut Machine,
        view_address: &'a mut usize,
        view_endian: &'a mut bool,
    ) -> Self {
        Self {
            machine,
            view_address,
            view_endian,
        }
    }
}
