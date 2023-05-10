use super::memory::ExMem;
use crate::pipeline::ForwardingUnit;
use crate::{Register, RegisterFile};
use anyhow::{bail, Result};

/// Struct representing this stages input
#[derive(Debug, Default, Clone)]
pub struct IdEx {
    // stage data
    pub alu_src: bool,
    pub reg_dst: bool,
    pub alu_op: u8,
    pub op_funct: u8,
    pub reg_1: u32,
    pub reg_2: u32,
    pub imm: u32,
    pub shamt: u32,
    pub rt: Register,
    pub rd: Register,
    pub syscall: bool,
    // forwarded data
    pub branch: bool,
    pub branch_not: bool,
    pub jump: bool,
    pub pc: u32,
    pub mem_write: bool,
    pub mem_read: bool,
    pub word_align: bool,
    pub mem_to_reg: bool,
    pub reg_write: bool,
    pub use_hilo: bool,
    pub rs: Register,

    // demo thing
    pub instruction: u32,
}

pub mod op_ctrl {
    pub const OP_R: u8 = 0;
    pub const OP_AND: u8 = 1;
    pub const OP_OR: u8 = 2;
    pub const OP_ADD: u8 = 3;
    pub const OP_SUB: u8 = 4;
    pub const OP_UPPER: u8 = 5;
}
use op_ctrl::*;

/// Runs execute stage
pub fn execute(input: IdEx, fwd_unit: ForwardingUnit) -> Result<ExMem> {
    let syscall = input.syscall;
    let mut use_shamt = false;
    // compute ALU control lines
    let alu_ctrl = match input.alu_op {
        OP_R => {
            // get info from instruction funct
            match input.op_funct {
                0x20 => (false, false, ALU_ADD), // add
                0x22 => (false, true, ALU_ADD),  // sub
                0x24 => (false, false, ALU_AND), // and
                0x2a => (false, false, ALU_SLT), // slt
                0x25 => (false, false, ALU_OR),  // or
                0x27 => (true, true, ALU_AND),   // nor
                0x0c => (false, false, ALU_ADD), // syscall
                0x06 => (false, false, ALU_SRL), // srlv
                0x26 => (false, false, ALU_XOR), // xor
                0x1c => (false, false, ALU_MUL), // mul
                0x1a => (false, false, ALU_DIV), // div

                0x10 | 0x12 => (false, false, ALU_ADD), // mflo & mfhi

                0x11 => {
                    // add.s
                    use_shamt = true;
                    (false, false, ALU_ADD_S)
                }
                0x00 => {
                    // sll
                    use_shamt = true;
                    (false, false, ALU_SLL)
                }
                0x02 => {
                    // srl
                    use_shamt = true;
                    (false, false, ALU_SRL)
                }
                0x03 => {
                    // sra
                    use_shamt = true;
                    (false, false, ALU_SRA)
                }
                _ => {
                    bail!("Unkown funct: {}", input.op_funct)
                }
            }
        }
        OP_ADD => (false, false, ALU_ADD),
        OP_SUB => (false, true, ALU_ADD),
        OP_AND => (false, false, ALU_AND),
        OP_OR => (false, false, ALU_OR),
        OP_UPPER => (false, false, ALU_UPPER),
        _ => {
            bail!("Unknown Instruction")
        }
    };

    // Handle ALU operation
    let mut arg1 = input.reg_1;
    let mut arg2 = input.reg_2;

    /*
    if fwd_unit.mem_wb.3 {
        println!("mem_wb: {:#x} {} {}", input.op_funct, input.rs, fwd_unit.mem_wb.1);
    }

    if fwd_unit.ex_mem.3 {
        println!("ex_mem: {:#x} {} {}", input.op_funct, input.rt, fwd_unit.ex_mem.1);
    }
     */

    // check forwarding unit on first register
    if fwd_unit.mem_wb.0 && input.rs == fwd_unit.mem_wb.1 {
        arg1 = fwd_unit.mem_wb.2.0;
    }
    if fwd_unit.ex_mem.0 && input.rs == fwd_unit.ex_mem.1 {
        arg1 = fwd_unit.ex_mem.2.0;
    }

    // check forwarding unit on second register
    if fwd_unit.mem_wb.0 && input.rt == fwd_unit.mem_wb.1 {
        arg2 = fwd_unit.mem_wb.2.0;
    }
    if fwd_unit.ex_mem.0 && input.rt == fwd_unit.ex_mem.1 {
        arg2 = fwd_unit.ex_mem.2.0;
    }

    // check forwarding unit on lo & hi registers
    match input.rs {
        HI => {
            if fwd_unit.mem_wb.3 {
                arg1 = fwd_unit.mem_wb.2.1;
            } else if fwd_unit.ex_mem.3 {
                arg1 = fwd_unit.ex_mem.2.1;
            }
        }

        LO => {
            if fwd_unit.mem_wb.3 {
                arg1 = fwd_unit.mem_wb.2.0;
            } else if fwd_unit.ex_mem.3 {
                arg1 = fwd_unit.ex_mem.2.0;
            }
        }

        _ => {}
    }

    let fwd_rt = arg2;

    // Handle immediate arguments
    if input.alu_src {
        arg2 = input.imm;
    }

    // check if we are using a shift operation.
    // and load the shamt if so
    if use_shamt {
        arg1 = arg2;
        arg2 = input.shamt;
    }

    let result = alu(arg1, arg2, alu_ctrl)?;

    Ok(ExMem {
        alu_result: result,
        zero: result.0 == 0,
        write_data: fwd_rt,
        write: input.mem_write,
        read: input.mem_read,
        word_align: input.word_align,
        mem_to_reg: input.mem_to_reg,
        write_register: if input.reg_dst { input.rd } else { input.rt },
        reg_write: input.reg_write,
        use_hilo: input.use_hilo,
        branch: input.branch,
        branch_not: input.branch_not,
        jump: input.jump,
        jump_pc: input.imm << 2,
        branch_pc: input.pc.wrapping_add((input.imm << 2) as i16 as u32) + 4, // casts are for sign extension
        syscall,
        instruction: input.instruction,
        pc: input.pc,
    })
}

pub mod alu_signals {
    //! ALU Controls
    pub const ALU_AND: u8 = 0;
    pub const ALU_OR: u8 = 1;
    pub const ALU_ADD: u8 = 2;
    pub const ALU_SLT: u8 = 3;
    pub const ALU_SLL: u8 = 4;
    pub const ALU_SRL: u8 = 5;
    pub const ALU_SRA: u8 = 6;
    pub const ALU_UPPER: u8 = 7;
    pub const ALU_XOR: u8 = 8;
    pub const ALU_ADD_S: u8 = 9;
    pub const ALU_MUL: u8 = 10;
    pub const ALU_DIV: u8 = 11;
}
use alu_signals::*;
use crate::Register::{HI, LO};

/// Simple ALU implementation.
/// TODO: Handle carry flag
pub fn alu(a: u32, b: u32, op: (bool, bool, u8)) -> Result<(u32, u32)> {
    //println!("{} {} {:?}", a, b, op);

    let a = if op.0 { !a } else { a };
    let b = if op.1 { !b } else { b };

    // this is a hack since we haven't implemented carry bits yet
    let arith_a = if op.0 { a.wrapping_add(1) } else { a };
    let arith_b = if op.1 { b.wrapping_add(1) } else { b };

    Ok(match op.2 {
        ALU_AND => (a & b, 0),
        ALU_OR => (a | b, 0),
        ALU_ADD => (arith_a.overflowing_add(arith_b).0, 0),
        ALU_SLL => (a.overflowing_shl(b).0, 0),
        ALU_XOR => (a ^ b, 0),

        ALU_MUL => (a * b, 0),
        ALU_DIV => (a / b, a % b),

        ALU_ADD_S => ((f32::from_bits(a) + f32::from_bits(b)).to_bits(), 0),

        // Rust uses signedness to select between logical and arithmetic right shifts
        ALU_SRL => (a.overflowing_shr(b).0, 0),
        ALU_SRA => ((a as i32).overflowing_shr(b).0 as u32, 0),

        ALU_UPPER => (b << 16, 0),

        ALU_SLT => (if a < b { 1 } else { 0 }, 0),
        _ => bail!("Unknown ALU instruction: {:?}", op),
    })
}
