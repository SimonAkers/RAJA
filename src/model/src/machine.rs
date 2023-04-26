use std::collections::HashMap;
use std::ops::ControlFlow;
use std::u32;

use anyhow::Result;

use crate::{Memory, parser::{
    self,
    model::{LabelTable, Line, Segment, Segments, STACK_BASE, TEXT_BASE},
}, pipeline::{self, PipelineState}, syscall::{resolve_syscall, Syscall}};

use crate::{RegisterFile, Register};

use crate::callback::Callback;
use crate::syscall::SyscallDiscriminants;

/// Represents an instance of a simulated MIPS computer.
#[derive(Default)]
pub struct Machine {
    pc: u32,
    regs: RegisterFile<u32>,
    state: PipelineState,
    memory: Memory,
    symbols: LabelTable,
    pending_syscall: Option<Syscall>,
    callbacks: HashMap<SyscallDiscriminants, Callback>,
    input: Option<String>,
}

impl Machine {
    /*
    /// Fetch a readonly view of this machines registers
    pub fn register(&self, reg: Register) -> u32 {
        self.regs.read_register(reg)
    }

    pub fn register_mut(&mut self, reg: Register) -> &mut u32 {
        self.regs.get_mut(reg)
    }
     */

    pub fn register_file(&self) -> &RegisterFile<u32> {
        &self.regs
    }

    pub fn read_word(&self, addr: u32) -> Result<u32> {
        self.memory.get(addr)
    }

    pub fn write_word(&mut self, addr: u32, val: u32) -> Result<()> {
        *self.memory.get_mut(addr)? = val;
        Ok(())
    }

    /// Reset this machine so it can be ran again
    ///
    /// Note that this will not reset the contents of memory or registers for that see
    /// [`hard_reset`]
    pub fn reset(&mut self) {
        self.pc = TEXT_BASE;
        self.state = PipelineState::default();
        self.regs = RegisterFile::default();
        self.pending_syscall = None;
        self.input = None;
    }

    /// Fully resets this machine including memory contents and registers
    pub fn hard_reset(&mut self) {
        self.memory = Memory::default();
        self.symbols = LabelTable::default();
        self.reset();
    }

    /// Set the contents of this machines memory to `mem`
    pub fn flash(&mut self, mem: Memory, syms: LabelTable) {
        self.memory = mem;
        self.symbols = syms;
    }

    /// Gets the current source code line
    pub fn current_line(&mut self) -> [Option<usize>; 5] {
        [
            self.symbols.get_line(self.state.if_id.pc),
            self.symbols.get_line(self.state.id_ex.pc),
            self.symbols.get_line(self.state.ex_mem.pc),
            self.symbols.get_line(self.state.mem_wb.pc),
            self.symbols.get_line(self.state.pipe_out.pc),
        ]
    }

    /// Gets the current pipeline stages
    pub fn pipeline(&self) -> &PipelineState {
        &self.state
    }

    /// Get the current contents of the stack
    pub fn stack(&mut self) -> Vec<(u32, u32)> {
        let sp = self.regs.value_or_default(Register::SP) / 4;
        let mut stack = vec![];
        for i in sp..STACK_BASE / 4 {
            let addr = i * 4;
            stack.push((addr, self.memory.get(addr).unwrap_or(0)));
        }
        stack
    }

    pub fn resolve_input(&mut self, input: &str) -> Result<()> {
        if let Some(syscall) = &self.pending_syscall {
            resolve_syscall(&mut self.regs, syscall, input)?;
            self.input = None;
        }
        Ok(())
    }

    /// Checks if there is a pending syscall
    ///
    /// # Returns
    /// True if there is a pending syscall
    pub fn pending_syscall(&self) -> bool {
        self.pending_syscall.is_some()
    }

    /**
    Steps the machine forward 1 CPU cycle.

    Returns ControlFlow::Break if the machine should stop cycling, otherwise ControlFlow::Continue.
     */
    pub fn cycle(&mut self) -> ControlFlow<()> {
        return match self.pending_syscall.clone() {
            None => {
                match pipeline::pipe_cycle(
                    &mut self.pc,
                    &mut self.regs,
                    &mut self.memory,
                    self.state.clone(),
                ) {
                    Ok((new_state, syscall)) => {
                        self.state = new_state;

                        if let Some(syscall) = syscall {
                            self.pending_syscall = Some(syscall);
                        }

                        ControlFlow::Continue(())
                    }

                    Err(err) => {
                        println!("{}", err.backtrace());
                        println!("{}", err.to_string());
                        ControlFlow::Break(())
                    }
                }
            }

            Some(syscall) => {
                self.handle_syscall(&syscall)
            }
        }
    }

    /**
    Handles a system call and returns a value indicating whether the machine should stop cycling.

    # Arguments
    - `syscall` - A borrowed reference to the system call to handle.

    Returns ControlFlow::Break if the machine should stop cycling, otherwise ControlFlow::Continue.
     */
    fn handle_syscall(&mut self, syscall: &Syscall) -> ControlFlow<()> {
        // Whether the syscall has been resolved (fully processed)
        let mut resolved = true;
        // Whether to run the callback for the given syscall
        let mut run_callback = true;

        // Handle calls internally and obtain any message to pass to callbacks
        let (flow, info) = match syscall {
            Syscall::Print(message) => (ControlFlow::Continue(()), Some(message)),
            Syscall::Error(message) => (ControlFlow::Break(()), Some(message)),
            Syscall::Quit => (ControlFlow::Break(()), None),

            // TODO: Consolidate to some function to make more readable
            Syscall::ReadInt | Syscall::ReadFloat => {
                match &self.input.clone() {
                    None => {
                        // No value is present, so stop cycling and mark syscall as unresolved
                        // Callback should put a value into the machine's input
                        resolved = false;
                        (ControlFlow::Break(()), None)
                    }

                    Some(input) => {
                        match self.resolve_input(input) {
                            Ok(_) => {
                                // Do not run callback since we already have data from frontend
                                run_callback = false;

                                // Continue cycling after reading in value
                                (ControlFlow::Continue(()), None)
                            }
                            Err(e) => {
                                println!("{}", e.to_string());
                                (ControlFlow::Break(()), None)
                            }
                        }
                    }
                }
            },

            _ => (ControlFlow::Break(()), None)
        };

        if resolved {
            self.pending_syscall = None;
        }

        if run_callback {
            match syscall {
                // Execute read callbacks
                Syscall::ReadInt | Syscall::ReadFloat => {
                    match self.callbacks.get_mut(&SyscallDiscriminants::ReadAny) {
                        None => (),
                        Some(mut callback) => callback.call(info),
                    }
                },

                // Execute other callbacks
                _ => {
                    match self.callbacks.get_mut(&SyscallDiscriminants::from(syscall)) {
                        None => (),
                        Some(mut callback) => callback.call(info),
                    }
                }
            }
        }

        flow
    }

    pub fn set_input(&mut self, input: Option<String>) {
        self.input = input;
    }

    pub fn get_callbacks(&mut self) -> &mut HashMap<SyscallDiscriminants, Callback> {
        &mut self.callbacks
    }
}

/// Method that create a memory instance from a script file
pub fn assembler(mut script: String) -> Result<(Memory, LabelTable)> {
    // Replace CRLF line endings
    script = script.replace("\r\n", "\n");
    // Ensure newline to prevent assembler error
    script.push('\n');

    // parse assembly
    let lines = parser::parse_string(&script)?;
    let labels = parser::compute_labels(&lines);

    // for each line in the parsed assembly assemble that line and add the result to a vec
    let mut memory = Memory::new();
    let mut segments = Segments::default();
    // current segement pc
    let mut pc = segments.switch(Segment::Text);
    for line in &lines {
        match line {
            Line::Instruction(ins) => {
                for word in ins {
                    let (bin, _) = word.asm(&labels, *pc);
                    //println!("{pc:X} {bin:X}\t{word:?}");
                    for byte in bin {
                        memory.set_byte(*pc, byte)?;
                        *pc += 1;
                    }
                }
            }
            Line::Segment(seg) => pc = segments.switch(*seg),
            _ => {}
        }
    }
    // insert guard instruction that causes the program to crash if it is encountered
    pc = segments.switch(Segment::Text);
    *memory.get_mut(*pc)? = 0x3402DEAD;
    *memory.get_mut(*pc + 4)? = 0xC;

    Ok((memory, labels))
}
