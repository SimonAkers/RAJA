use crate::{stages::execute::{op_ctrl::*, IdEx}, Register, RegisterFile, Align, opcode};
use anyhow::{bail, Result};
use crate::Register::ZERO;

// Struct representing this stages inputs
#[derive(Debug, Default, Clone)]
pub struct IfId {
    pub instruction: u32,
    pub pc: u32,
}

/// Decodes and instruction
pub fn decode(reg_file: &mut RegisterFile<u32>, input: IfId) -> Result<IdEx> {
    // instruction masks
    let fn_mask = 0b00000000000000000000000000111111;
    let sh_mask = 0b00000000000000000000011111000000;
    let rd_mask = 0b00000000000000001111100000000000;
    let rt_mask = 0b00000000000111110000000000000000;
    let rs_mask = 0b00000011111000000000000000000000;
    let op_mask = 0b11111100000000000000000000000000;
    let j_mask = 0b00000011111111111111111111111111;
    let imm_mask = fn_mask | sh_mask | rd_mask;

    //println!("Instruction: {:032b}", input.instruction);

    // Use masks to get the field values
    let mut rd = (input.instruction & rd_mask) >> 11;
    let mut rt = (input.instruction & rt_mask) >> 16;
    let mut rs = (input.instruction & rs_mask) >> 21;
    let funct = input.instruction & fn_mask;
    let shamt = (input.instruction & sh_mask) >> 6;
    let op = (input.instruction & op_mask) >> 26;
    let mut imm = input.instruction & imm_mask;
    let j_imm = input.instruction & j_mask;

    // sign extend the imm value
    imm = ((imm << 16) as i32 >> 16) as u32;

    // select float registers if floating point funct
    if funct == 0x11 {
        rd += 32;
        rt += 32;
        rs += 32;
    }

    // make registers typed
    let mut rs: Register = rs.into();
    let mut rt: Register = rt.into();
    let mut rd: Register = rd.into();

    // read rs and rt
    let mut read_rs = reg_file.value_or_default(rs);
    let mut read_rt = reg_file.value_or_default(rt);

    // handle controls
    let mut reg_dst; // determines destination register (0: rt, 1: rd)
    let mut alu_src; // if enabled use immediate value as alu arg2
    let mut mem_to_reg; // if enabled dest register gets a memory location otheriwse gets alu result
    let mut reg_write; // if disabled don't write to dest register
    let mut mem_write; // if enabled write to alu result
    let mut mem_read; // if enabled read from alu result
    let mut alu_op; // alu operation
    let mut branch; // enable branching
    let mut branch_not; // enable branch not equal
    let mut jump; // enable jumping
    let mut link = false; // whether to link for return
    let mut syscall = false;
    let mut word_align = true;

    // This is where instructions are defined
    match op {
        0 | 1 | 0x1c | 0x2a => {
            syscall = funct == 0x0c;
            // R-type instruction
            reg_dst = true;
            alu_src = false;
            mem_to_reg = false;
            reg_write = true;
            mem_read = false;
            mem_write = false;
            branch = false;
            branch_not = false;
            jump = false;
            alu_op = OP_R;
        }
        0x20 => {
            // LB instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = true;
            reg_write = true;
            mem_read = true;
            mem_write = false;
            word_align = false;
            branch = false;
            branch_not = false;
            jump = false;
            alu_op = OP_ADD;
        }
        0x28 => {
            // SB instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = false;
            reg_write = false;
            mem_read = false;
            mem_write = true;
            word_align = false;
            branch = false;
            branch_not = false;
            jump = false;
            alu_op = OP_ADD;
        }
        0x23 => {
            // LW instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = true;
            reg_write = true;
            mem_read = true;
            mem_write = false;
            branch = false;
            branch_not = false;
            jump = false;
            alu_op = OP_ADD;
        }
        0x2b => {
            // SW instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = false;
            reg_write = false;
            mem_read = false;
            mem_write = true;
            branch = false;
            branch_not = false;
            jump = false;
            alu_op = OP_ADD;
        }
        0x8 => {
            // ADDI instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = false;
            reg_write = true;
            mem_read = false;
            mem_write = false;
            branch = false;
            branch_not = false;
            jump = false;
            alu_op = OP_ADD;
        }

        0xc => {
            // ANDI instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = false;
            reg_write = true;
            mem_read = false;
            mem_write = false;
            branch = false;
            branch_not = false;
            jump = false;
            alu_op = OP_AND;
        }

        0xf => {
            // LUI instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = false;
            reg_write = true;
            mem_write = false;
            mem_read = false;
            alu_op = OP_UPPER;
            branch = false;
            branch_not = false;
            jump = false;
        }

        0xd => {
            // ORI instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = false;
            reg_write = true;
            mem_read = false;
            mem_write = false;
            branch = false;
            branch_not = false;
            jump = false;
            alu_op = OP_OR;
        }
        0x4 => {
            // BEQ instruction
            reg_dst = false;
            alu_src = false;
            mem_to_reg = false;
            reg_write = false;
            mem_read = false;
            mem_write = false;
            branch = true;
            branch_not = false;
            jump = false;
            alu_op = OP_SUB;
        }
        0x5 => {
            // BNE instruction
            reg_dst = false;
            alu_src = false;
            mem_to_reg = false;
            reg_write = false;
            mem_read = false;
            mem_write = false;
            branch = true;
            branch_not = true;
            jump = false;
            alu_op = OP_SUB;
        }
        0x02 | 0x03 => {
            // J instruction
            reg_dst = false;
            alu_src = false;
            mem_to_reg = false;
            reg_write = false;
            mem_read = false;
            mem_write = false;
            branch = false;
            branch_not = false;
            jump = true;
            alu_op = OP_ADD;
            imm = j_imm;

            if op == 0x03 {
                reg_dst = true;
                reg_write = true;
                rs = ZERO;
                rt = ZERO;
                rd = Register::RA;
                read_rs = input.pc;
                read_rt = 4;
            }
        }
        _ => {
            bail!("Unrecognized instruction opcode 0x{:x}", op)
        }
    }

    if op == 0 && funct == 0x8 {
        // JR instruction
        reg_dst = false;
        alu_src = false;
        mem_to_reg = false;
        reg_write = false;
        mem_read = false;
        mem_write = false;
        branch = false;
        branch_not = false;
        jump = true;
        alu_op = OP_ADD;
        imm = read_rs;
    }

    Ok(IdEx {
        alu_src,
        reg_dst,
        alu_op,
        op_funct: funct as u8,
        reg_1: read_rs,
        reg_2: read_rt,
        imm,
        shamt,
        rt,
        rs,
        rd,
        mem_write,
        mem_read,
        word_align,
        mem_to_reg,
        reg_write,
        branch,
        branch_not,
        jump,
        link,
        pc: input.pc,
        syscall,
        instruction: input.instruction,
    })
}
