use std::fs::read;
use super::writeback::MemWb;
use crate::{Memory, Register};
use anyhow::{Context, Result};

/// Struct representing this stages input
#[derive(Debug, Default, Clone)]
pub struct ExMem {
    // stage data
    pub alu_result: (u32, u32),
    pub zero: bool,
    pub branch: bool,
    pub branch_not: bool,
    pub jump: bool,
    pub write_data: u32,
    pub write: bool,
    pub read: bool,
    pub word_align: bool,
    pub branch_pc: u32,
    pub jump_pc: u32,
    // forwarded data
    pub mem_to_reg: bool,
    pub write_register: Register,
    pub reg_write: bool,
    pub use_hilo: bool,
    pub syscall: bool,

    // demo thing
    pub instruction: u32,
    pub pc: u32,
}

/// Memory access pipeline stage
pub fn memory(pc: &mut u32, memory: &mut Memory, input: ExMem) -> Result<MemWb> {
    let mut read_data = 0;

    // handle memory accesses
    if input.write {
        if input.word_align {
            *memory.get_mut(input.alu_result.0)? = input.write_data;
        } else {
            memory.set_byte(input.alu_result.0, input.write_data as u8)?;
        }
        //println!("writing: {} to {:#x}", input.write_data, input.alu_result);
    }
    if input.read {
        if input.word_align {
            read_data = memory.get(input.alu_result.0)?;//.context("In memory stage")?;
        } else {
            read_data = memory.get_byte(input.alu_result.0)? as u32;
        }
        //println!("reading: {} from {:#x}", input.write_data, input.alu_result);
    }

    if input.branch {
        if (!input.branch_not && input.zero) || (input.branch_not && !input.zero) {
            // branch to PC copmuted in execute stage
            *pc = input.branch_pc;
        }
    }

    if input.jump {
        *pc = input.jump_pc;
    }

    Ok(MemWb {
        mem_to_reg: input.mem_to_reg,
        mem_data: read_data,
        alu_data: input.alu_result,
        write_register: input.write_register,
        reg_write: input.reg_write,
        use_hilo: input.use_hilo,
        syscall: input.syscall,
        instruction: input.instruction,
        pc: input.pc,
    })
}
