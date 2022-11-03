use model::machine::Machine;
use view::traits::AppUI;

pub struct App {
    ui: Box<dyn AppUI>,
    machine: Machine
}

impl App {
    pub fn new(ui: Box<dyn AppUI>) -> Self {
        // Create a new instance of App and connect the UI
        let app = Self { ui, machine: Default::default() };
        //app.connect_ui();

        app
    }

    pub fn start(&self) {
        self.ui.start();
    }

    fn connect_ui(&self) {
        let src = self.ui.get_source();
    }
}