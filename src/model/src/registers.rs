/** A list of MIPS integer registers in order of index */
pub const INT_REGS_ORDERED: &'static [&'static str] = &[
    "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3",
    "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
    "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
    "t8", "t9", "k0", "k1", "gp", "sp", "fp", "ra",
];

pub struct Register<T> {
    name: &'static str,
    value: T,
}

impl<T: Default> Register<T> {
    pub fn new(name: &str) -> Self {
        Self { name, value: T::default() }
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }

    pub fn value(&self) -> T {
        &self.value
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

pub struct RegisterFile<T> {
    registers: Vec<Register<T>>,
}

impl<T> RegisterFile<T> {
    pub fn add_register(&mut self, name: &str) {
        self.registers.push(Register::new(name));
    }

    pub fn register(&self, index: usize) -> Register<T> {
        /*
        match self.registers.get(index) {
            None => {}
            Some(_) => {}
        }

         */

        todo!()
    }
}

impl Default for RegisterFile<i32> {
    fn default() -> Self {
        let mut reg_file = Self { registers: Vec::new() };

        for reg in INT_REGS_ORDERED {
            reg_file.add_register(reg);
        }



        reg_file
    }
}