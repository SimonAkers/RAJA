use crate::{Register, RegisterFile};

/// struct representing this structs input
#[derive(Debug, Default, Clone)]
pub struct MemWb {
    pub mem_to_reg: bool,
    pub mem_data: u32,
    pub alu_data: (u32, u32),
    pub write_register: Register,
    pub reg_write: bool,
    pub use_hilo: bool,
    pub syscall: bool,
    // demo thing
    pub instruction: u32,
    pub pc: u32,
}

#[derive(Debug, Default, Clone)]
pub struct PipelineOutput {
    pub syscall: bool,
    pub instruction: u32,
    pub pc: u32,
}

/// Writeback pipeline stage
pub fn writeback(reg_file: &mut RegisterFile<u32>, input: MemWb) -> PipelineOutput {
    if input.reg_write {
        if input.mem_to_reg {
            reg_file.set_value(input.write_register, input.mem_data);
        } else {
            reg_file.set_value(input.write_register, input.alu_data.0);
        }
    } else if input.use_hilo {
        let (lo, hi) = input.alu_data;
        reg_file.set_value(Register::LO, lo);
        reg_file.set_value(Register::HI, hi);
    }

    PipelineOutput {
        syscall: input.syscall,
        instruction: input.instruction,
        pc: input.pc,
    }
}
