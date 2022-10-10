use gtk::gio;

fn main() {
    gio::compile_resources(
        "src/view/res",
        "src/view/res/resources.gresource.xml",
        "app_window.gresource"
    );
}