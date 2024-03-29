use crate::stages;
use crate::stages::execute::IdEx;
use crate::stages::inputs::*;
use crate::stages::writeback::PipelineOutput;
use crate::syscall::{handle_syscall, Syscall};
use crate::{Memory, Register, RegisterFile};

use anyhow::Result;

/// This is a simple function to single step the CPU.
///
/// Eventually this should pipeline data instead of doing an entire instruction each cycle but that
/// can't be done until we fix all the data and control hazard issues.
pub fn _single_cycle(pc: &mut u32, regs: &mut RegisterFile<u32>, mem: &mut Memory) -> Option<Syscall> {
    // should never forward
    let fwd_unit = ForwardingUnit {
        ex_mem: (false, Register::ZERO, (0, 0), false),
        mem_wb: (false, Register::ZERO, (0, 0), false),
    };

    let if_id = stages::fetch(pc, mem);
    let id_ex = stages::decode(regs, if_id.unwrap());
    let ex_mem = stages::execute(id_ex.unwrap(), fwd_unit);
    let mem_wb = stages::memory(pc, mem, ex_mem.unwrap()).unwrap();
    let pipe_out = stages::writeback(regs, mem_wb);

    // pretend we jumped to the syscall vector
    if pipe_out.syscall {
        match handle_syscall(regs, mem) {
            Ok(syscall) => Some(syscall),
            Err(e) => Some(Syscall::Error(format!("{}", e))),
        }
    } else {
        None
    }
}

#[derive(Default, Debug, Clone)]
pub struct PipelineState {
    pub if_id: IfId,
    pub id_ex: IdEx,
    pub ex_mem: ExMem,
    pub mem_wb: MemWb,
    pub pipe_out: PipelineOutput,
}

#[derive(Debug, Clone, Copy)]
pub struct ForwardingUnit {
    pub ex_mem: (bool, Register, (u32, u32), bool),
    pub mem_wb: (bool, Register, (u32, u32), bool),
}

/// Steps the machine forward in a pipelined manner.
///
/// Returns the current state of all pipeline stages after stepping the machine forward 1 stage.
/// Pass that state back into this function to continue stepping the machine forward
pub fn pipe_cycle(
    pc: &mut u32,
    regs: &mut RegisterFile<u32>,
    mem: &mut Memory,
    state: PipelineState,
) -> Result<(PipelineState, Option<Syscall>)> {
    // contruct forwarding unit
    let fwd_unit = ForwardingUnit {
        ex_mem: (
            state.ex_mem.reg_write,
            state.ex_mem.write_register,
            state.ex_mem.alu_result,
            state.ex_mem.use_hilo,
        ),
        mem_wb: (
            state.mem_wb.reg_write,
            state.mem_wb.write_register,
            if state.mem_wb.mem_to_reg {
                (state.mem_wb.mem_data, 0)
            } else {
                state.mem_wb.alu_data
            },
            state.mem_wb.use_hilo,
        ),
    };

    let pipe_out = stages::writeback(regs, state.mem_wb);

    // pretend we jumped to the syscall vector
    if pipe_out.syscall {
        let syscall =
            Some(handle_syscall(regs, mem).unwrap_or_else(|e| Syscall::Error(format!("{}", e))));
        // stall in case of syscall
        // TODO: Maybe not the best solution but ¯\_(ツ)_/¯
        return Ok((
            PipelineState {
                pipe_out,
                mem_wb: MemWb::default(),
                ..state
            },
            syscall,
        ));
    }

    let mem_wb = stages::memory(pc, mem, state.ex_mem.clone())?;

    let ex_mem = stages::execute(state.id_ex.clone(), fwd_unit)?;

    // stall in case of syscall
    // TODO: Maybe not the best solution but ¯\_(ツ)_/¯
    if ex_mem.syscall || mem_wb.syscall {
        return Ok((
            PipelineState {
                if_id: state.if_id,
                id_ex: IdEx::default(),
                ex_mem,
                mem_wb,
                pipe_out,
            },
            None,
        ));
    }
    let id_ex = stages::decode(regs, state.if_id.clone())?;
    // hazard detector
    if state.id_ex.mem_read {
        if state.id_ex.rt == id_ex.rs {
            return Ok((
                PipelineState {
                    if_id: state.if_id,
                    id_ex: IdEx::default(),
                    ex_mem,
                    mem_wb,
                    pipe_out,
                },
                None,
            ));
        }
        if state.id_ex.rt == id_ex.rt {
            return Ok((
                PipelineState {
                    if_id: state.if_id,
                    id_ex: IdEx::default(),
                    ex_mem,
                    mem_wb,
                    pipe_out,
                },
                None,
            ));
        }
    }

    let if_id = stages::fetch(pc, mem);

    Ok((
        PipelineState {
            if_id: if_id?,
            id_ex,
            ex_mem,
            mem_wb,
            pipe_out,
        },
        None,
    ))
}
