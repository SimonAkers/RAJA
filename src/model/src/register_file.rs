use std::hash::Hash;
use std::str::FromStr;

use indexmap::IndexMap;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

use crate::register_file::Register::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, EnumString, EnumIter)]
pub enum Register {
    ZERO = 0,
    AT = 1,
    V0 = 2,
    V1 = 3,
    A0 = 4,
    A1 = 5,
    A2 = 6,
    A3 = 7,
    T0 = 8,
    T1 = 9,
    T2 = 10,
    T3 = 11,
    T4 = 12,
    T5 = 13,
    T6 = 14,
    T7 = 15,
    S0 = 16,
    S1 = 17,
    S2 = 18,
    S3 = 19,
    S4 = 20,
    S5 = 21,
    S6 = 22,
    S7 = 23,
    T8 = 24,
    T9 = 25,
    K0 = 26,
    K1 = 27,
    GP = 28,
    SP = 29,
    FP = 30,
    RA = 31,
    #[default]
    UNKNOWN = 999,
}

impl Register {
    pub fn value(&self) -> i32 {
        *self as i32
    }
}

impl From<String> for Register {
    fn from(value: String) -> Self {
        match Register::from_str(value.as_str()) {
            Ok(register) => register,
            Err(_) => UNKNOWN,
        }
    }
}

impl From<i32> for Register {
    fn from(value: i32) -> Self {
        for reg in Register::iter() {
            if reg.value() == value {
                return reg;
            }
        }

        UNKNOWN
    }
}

impl From<Register> for String {
    fn from(value: Register) -> Self {
        value.into()
    }
}

/** An ordered list of MIPS integer register names */
pub const INT_REGS_ORDERED: &[&str] = &[
    "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3",
    "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
    "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
    "t8", "t9", "k0", "k1", "gp", "sp", "fp", "ra",
];

/** An ordered list of MIPS floating point register names */
pub const FLOAT_REGS_ORDERED: &[&str] = &[
    "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7",
    "f8", "f9", "f10", "f11", "f12", "f13", "f14", "f15",
    "f16", "f17", "f18", "f19", "f20", "f21", "f22", "f23",
    "f24", "f25", "f26", "f27", "f28", "f29", "f30", "f31",
];

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
    - `name` - The name of the register.
    - `value` - The value to store in the register.
     */
    pub fn set_value<S: Into<String>>(&mut self, register_name: S, value: T) {
        self.registers.insert(register_name.into(), value);
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
Implementation of From<Vec<&str>> for types that implement Default. This allows a RegisterFile
to be constructed from an ordered list of register names.
 */
impl<T: Default> From<Vec<&str>> for RegisterFile<T> {
    fn from(names: Vec<&str>) -> Self {
        let mut reg_file = Self { registers: IndexMap::new() };

        // Add an empty register for each register name
        for name in names {
            reg_file.set_default(name);
        }

        reg_file
    }
}

/** Implementation of Default for RegisterFile<i32>. */
impl Default for RegisterFile<i32> {
    fn default() -> Self {
        // Create a RegisterFile from the list of integer register names
        RegisterFile::from(INT_REGS_ORDERED.to_vec())
    }
}

/** Implementation of Default for RegisterFile<f32>. */
impl Default for RegisterFile<f32> {
    fn default() -> Self {
        // Create a RegisterFile from the list of floating point register names
        RegisterFile::from(FLOAT_REGS_ORDERED.to_vec())
    }
}