use std::hash::Hash;

use indexmap::IndexMap;
use strum::IntoEnumIterator;
use crate::model::STACK_BASE;
use crate::Register;

/**
A struct wrapping the IndexMap type, representing a register file for storing generic types.

IndexMap is used instead of HashMap to allow registers to be referenced both by name and by index.
 */
pub struct RegisterFile<T> {
    registers: IndexMap<String, T>,
}

/** Generic implementation of RegisterFile. */
impl<T> RegisterFile<T> {
    /**
    Sets the value of a given register.

    # Arguments
    - `register` - The register to set.
    - `value` - The value to store in the register.
     */
    pub fn set_value<R, V>(&mut self, register: R, value: V)
        where
            R: Into<Register>,
            V: Into<T>,
    {
        match self.registers.get_index(register.into().into()) {
            None => {}
            Some(reg_info) => {
                self.registers.insert(reg_info.0.into(), value.into());
            }
        }
    }

    /**
    Gets the value of a given register.

    # Arguments
    - `name` - The name of the register to read from.

    # Returns
    - An Option containing the value held by the register, or None if there is no
    value to obtain.
     */
    pub fn value<S: Into<String> + Hash + Eq>(&self, name: S) -> Option<&T> {
        self.registers.get(&name.into())
    }

    /**
    Gets the value of a given register by its index.

    # Arguments
    - `index` - The index of the register to read from.

    # Returns
    - An Option containing a tuple of the register name and value held by the register,
    or None if there is no tuple value to obtain.
     */
    pub fn value_by_index(&self, index: usize) -> Option<(&String, &T)> {
        self.registers.get_index(index)
    }

    pub fn map(&self) -> &IndexMap<String, T> {
        &self.registers
    }
}

/** Implementation for types that implement Default. */
impl<T: Default> RegisterFile<T> {
    /**
    Sets the entry for a register of a given name to the type's default value.

    This can be used to reset a register or "add" a register to the file.

    # Arguments
    - `name` - The name of the register to set to default.
     */
    pub fn set_default<S: Into<String>>(&mut self, name: S) {
        self.registers.insert(name.into(), T::default());
    }

    /**
    Gets the value of a given register or the default value if there is no value to obtain.

    # Arguments
    - `name` - The name of the register to read from.

    # Returns
    - The value of a given register or the default value if there is no value to obtain.
     */
    pub fn value_or_default<S: Into<String> + Hash + Eq>(&self, name: S) -> T where T: Clone {
        match self.value(name.into()) {
            None => T::default(),
            Some(value) => value.clone(),
        }
    }

    /**
    Gets the value of a given register by its index or the default value if there is no
    value to obtain.

    # Arguments
    - `index` - The index of the register to read from.

    # Returns
    - A tuple containing the name and value of a given register, or a placeholder name and the
    default value if there is no value to obtain.
     */
    pub fn value_or_default_by_index(&self, index: usize) -> (String, T) where T: Clone {
        match self.value_by_index(index) {
            None => ("unknown".to_owned(), T::default()),
            Some(value) => (value.0.clone(), value.1.clone()),
        }
    }
}

/**
Implementation of From<Vec<S: Into<String>> for types that implement Default. This allows a RegisterFile
to be constructed from an ordered list of register names.
 */
impl<T: Default, S: Into<String>> From<Vec<S>> for RegisterFile<T> {
    fn from(names: Vec<S>) -> Self {
        let mut reg_file = Self { registers: IndexMap::new() };

        // Add an empty register for each register name
        for name in names {
            reg_file.set_default(name.into());
        }

        reg_file
    }
}

/** Implementation of Default for RegisterFile<u32>. */
impl Default for RegisterFile<u32> {
    fn default() -> Self {
        let mut names: Vec<String> = Vec::new();

        for reg in Register::iter() {
            names.push(reg.to_string().to_lowercase());
        }

        // Create a RegisterFile from the list of register names
        let mut reg_file = RegisterFile::from(names);
        reg_file.set_value(Register::SP, STACK_BASE);

        reg_file
    }
}