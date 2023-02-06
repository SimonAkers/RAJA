use std::ops::ControlFlow;
use crate::syscall::Syscall;

pub struct SyscallHandlers {
    print: Option<dyn FnOnce(&Syscall) -> ControlFlow<()>>,
    error: Option<dyn FnOnce(&Syscall) -> ControlFlow<()>>,
}

impl SyscallHandlers {
    pub fn get_handler(self, syscall: &Syscall) -> &Option<&dyn FnOnce(&Syscall) -> ControlFlow<()>> {
        match syscall {
            Syscall::Print(_) => &self.print,
            Syscall::Error(_) => &self.error,
            Syscall::Quit => None,
            Syscall::ReadInt => None,
        }
    }
}