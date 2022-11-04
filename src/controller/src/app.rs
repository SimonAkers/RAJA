use model::machine::Machine;
use view::traits::AppUI;

pub struct App {
    ui: Box<dyn AppUI>,
    machine: Machine
}

impl App {
    pub fn new(ui: Box<dyn AppUI>) -> Self {
        // Create a new instance of App
        Self { ui, machine: Default::default() }
    }

    pub fn start(&self) {
        self.ui.start();
    }

    fn connect(&self) {
        let src = self.ui.get_source();
    }
}