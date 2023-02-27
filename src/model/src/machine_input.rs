
// Maybe use vectors instead??
#[derive(Default)]
pub struct MachineInput {
    integer: Option<i32>,
    float: Option<f32>,
    double: Option<f64>,
    string: Option<String>,
}

impl MachineInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn integer(&self) -> Option<i32> {
        self.integer
    }

    pub fn float(&self) -> Option<f32> {
        self.float
    }

    pub fn double(&self) -> Option<f64> {
        self.double
    }

    pub fn string(&self) -> Option<String> {
        self.string.clone()
    }

    pub fn set_integer(&mut self, integer: Option<i32>) {
        self.integer = integer;
    }

    pub fn set_float(&mut self, float: Option<f32>) {
        self.float = float;
    }

    pub fn set_double(&mut self, double: Option<f64>) {
        self.double = double;
    }

    pub fn set_string(&mut self, string: Option<String>) {
        self.string = string;
    }

    pub fn flush(&mut self) {
        self.set_integer(None);
        self.set_float(None);
        self.set_double(None);
        self.set_string(None);
    }
}