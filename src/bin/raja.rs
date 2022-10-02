use controller::app::App;
use view::app_gui::{AdwGUI, AppGUI};

fn main() {
    let gui = AdwGUI::new();
    let app = App::new(Box::new(gui));

    app.run();
}