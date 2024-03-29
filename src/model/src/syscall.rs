use crate::{Memory, RegisterFile, Register};
use anyhow::{bail, Context, Result};
use strum_macros::EnumDiscriminants;

#[derive(Debug, Default, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(Hash))]
pub enum Syscall {
    Print(String),
    Error(String),
    #[default]
    Quit,
    ReadAny,
    ReadInt,
    ReadFloat,
    ReadString,
}

pub fn resolve_syscall(reg_file: &mut RegisterFile<u32>, mem: &mut Memory, syscall: &Syscall, value: &str) -> Result<()> {
    match syscall {
        Syscall::ReadInt => {
            let buffer = value.trim();
            let val = buffer
                .parse::<i32>()
                .with_context(|| format!("Attempting to parse '{}'", buffer))?;

            reg_file.set_value(Register::V0, val as u32);
        },

        Syscall::ReadFloat => {
            let buffer = value.trim();

            let val = buffer
                .parse::<f32>()
                .with_context(|| format!("Attempting to parse '{}'", buffer))?;

            reg_file.set_value(Register::F0, val.to_bits());
        },

        Syscall::ReadString => {
            let buffer = value.trim().to_string().into_bytes();

            let address = reg_file.value_or_default(Register::A0);
            let buf_size = reg_file.value_or_default(Register::A1);

            for i in 0..buf_size {
                let byte = buffer.get(i as usize).unwrap_or(&0_u8).clone();
                mem.set_byte(address + i, byte)?;
            }
        },

        _ => {}
    }
    Ok(())
}

pub fn handle_syscall(reg_file: &mut RegisterFile<u32>, mem: &mut Memory) -> Result<Syscall> {
    // Handle syscall instructions
    let v0 = reg_file.value_or_default(Register::V0);
    match v0 {
        1 => {
            // print int
            let arg = reg_file.value_or_default(Register::A0);
            Ok(Syscall::Print(format!("{}", arg as i32)))
        }
        2 => {
            // print float
            let arg = reg_file.value_or_default(Register::F12);
            Ok(Syscall::Print(format!("{}", f32::from_bits(arg))))
        }
        4 => {
            // print string
            let mut ptr = reg_file.value_or_default(Register::A0);

            //println!("SYSCALL 4 {ptr}");
            // to make this unicode aware we need to bundle it into a buffer first
            let mut buffer = vec![];
            let mut b = mem.get_byte(ptr)?;
            while b != 0 {
                buffer.push(b);
                ptr += 1;
                b = mem.get_byte(ptr)?;
            }
            let s = String::from_utf8(buffer)?;
            Ok(Syscall::Print(format!("{}", s)))
        }

        5 => Ok(Syscall::ReadInt),
        6 => Ok(Syscall::ReadFloat),
        8 => Ok(Syscall::ReadString),
        10 => Ok(Syscall::Quit),

        11 => {
            // print char
            let arg = reg_file.value_or_default(Register::A0);
            let c = char::from_u32(arg).unwrap_or('�');
            Ok(Syscall::Print(format!("{}", c)))
        }
        12 => {
            bail!("read char syscall not yet implemented");
            // implementing this properly will require a single point for handling stdin
        }
        34 => {
            // print int hex
            let arg = reg_file.value_or_default(Register::A0);
            Ok(Syscall::Print(format!("{:x}", arg)))
        }
        35 => {
            // print int binary
            let arg = reg_file.value_or_default(Register::A0);
            Ok(Syscall::Print(format!("{:b}", arg)))
        }
        36 => {
            // print int unsigned
            let arg = reg_file.value_or_default(Register::A0);
            Ok(Syscall::Print(format!("{}", arg)))
        }
        0xFFFFDEAD => {
            // failed to exit kernel error
            Ok(Syscall::Error(format!(
                "program finished (ran into kernel)"
            )))
        }
        _ => {
            bail!("Unrecognized syscall: {}", v0)
        }
    }
}
