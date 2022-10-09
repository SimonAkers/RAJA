use view::traits::AppUI;

pub struct App {
    gui: Box<dyn AppUI>
}

impl App {
    pub fn new(gui: Box<dyn AppUI>) -> Self {
        Self { gui }
    }

    pub fn run(&self) {
        self.gui.run();
    }
}