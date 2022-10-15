use crate::{parser::opcode_name, Machine};

pub struct PipelineView<'a> {
    machine: &'a mut Machine,
}

impl<'a> PipelineView<'a> {
    pub fn new(machine: &'a mut Machine) -> Self {
        Self { machine }
    }
}