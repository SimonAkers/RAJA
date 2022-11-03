use controller::app::App;
use view::adw_gui::AdwGUI;

fn main() {
    let gui = AdwGUI::new();
    let app = App::new(Box::new(gui));

    app.start();
}