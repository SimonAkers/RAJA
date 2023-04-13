use anyhow::Result;

use crate::{parser::int, Machine, Register};

use super::console::Console;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum WatchType {
    Register,
    Memory,
}

impl Default for WatchType {
    fn default() -> Self {
        Self::Register
    }
}

impl WatchType {
    /// The pretty name of this type of watch
    pub fn label(&self) -> &str {
        match self {
            WatchType::Register => "Register",
            WatchType::Memory => "Memory",
        }
    }
}

/// Represents a single watch allows its contents to be read and mutated
#[derive(Default, Clone, Copy)]
pub struct Watch {
    ty: WatchType,
    val: u32,
}

/*
impl Watch {
    /// Read the contents of this watch
    pub fn read(&self, vm: &Machine) -> u32 {
        match self.ty {
            WatchType::Register => vm.register(self.val.into()),
            WatchType::Memory => vm.read_word(self.val).unwrap_or(0),
        }
    }

    /// Write a value into the contents of this watch
    pub fn write(&self, vm: &mut Machine, val: u32) -> Result<()> {
        match self.ty {
            WatchType::Register => *vm.register_mut(self.val.into()) = val,
            WatchType::Memory => vm.write_word(self.val, val)?,
        }
        Ok(())
    }
}
 */

/// Displays a single watch and allows it and its contents to be mutated
pub struct WatchView<'a> {
    watch: &'a mut Watch,
    vm: &'a mut Machine,
    id: usize,
    console: &'a mut Console,
}

impl<'a> WatchView<'a> {
    pub fn new(
        watch: &'a mut Watch,
        vm: &'a mut Machine,
        id: usize,
        console: &'a mut Console,
    ) -> Self {
        Self {
            watch,
            vm,
            id,
            console,
        }
    }
}

/// Draws a list of watches and allows new watches to be added to the list
pub struct WatchList<'a> {
    watches: &'a mut Vec<Watch>,
    vm: &'a mut Machine,
    console: &'a mut Console,
}

impl<'a> WatchList<'a> {
    pub fn new(watches: &'a mut Vec<Watch>, vm: &'a mut Machine, console: &'a mut Console) -> Self {
        Self {
            watches,
            vm,
            console,
        }
    }
}
