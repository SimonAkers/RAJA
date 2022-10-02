use view::app_gui::AppGUI;

pub struct App {
    gui: Box<dyn AppGUI>
}

impl App {
    pub fn new(gui: Box<dyn AppGUI>) -> Self {
        Self { gui }
    }

    pub fn run(&self) {
        self.gui.run();
    }
}