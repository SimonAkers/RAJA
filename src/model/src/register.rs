use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};
use crate::register::Register::UNKNOWN;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, EnumString, EnumIter)]
#[strum(serialize_all = "mixed_case")]
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
    pub fn id(&self) -> u32 {
        *self as u32
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
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

impl From<Register> for String {
    fn from(value: Register) -> Self {
        value.to_string().to_lowercase()
    }
}

impl From<Register> for u32 {
    fn from(value: Register) -> Self {
        value.id()
    }
}

impl From<Register> for usize {
    fn from(value: Register) -> Self {
        value.id() as usize
    }
}

impl From<u32> for Register {
    fn from(value: u32) -> Self {
        for reg in Register::iter() {
            if reg.id() == value {
                return reg;
            }
        }

        UNKNOWN
    }
}