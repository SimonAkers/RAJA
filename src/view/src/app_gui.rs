use gtk::prelude::*;
use sourceview5::prelude::*;

use adw::{Application, ApplicationWindow, ColorScheme, StyleManager};
use gtk::{Builder, CssProvider, StyleContext, Widget};
use gtk::gdk::Display;

pub struct AppGUI {
    adw_app: Application,
}

impl AppGUI {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id("net.shayes.raja")
            .flags(Default::default())
            .build();

        sourceview5::View::ensure_type();
        sourceview5::Buffer::ensure_type();
        sourceview5::Language::ensure_type();

        app.connect_startup(|_| load_css());
        app.connect_activate(build_from_xml);

        Self { adw_app: app }
    }

    pub fn run(&self) {
        self.adw_app.run();
    }
}

impl Default for AppGUI {
    fn default() -> Self {
        AppGUI::new()
    }
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("../res/style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_from_xml(app: &Application) {
    StyleManager::default().set_color_scheme(ColorScheme::Default);

    // Build UI from the specification
    let builder = Builder::from_file("src/view/res/ui/main.ui");

    // Style the source view
    let srcview: sourceview5::View = builder.object("source_view").unwrap();
    style_srcview(&srcview);

    // Get the main widget from the builder
    let content: Widget = builder.object("main_box").unwrap();

    // Create the window
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(960)
        .default_height(540)
        .width_request(400)
        .height_request(300)
        .title("RAJA")
        .content(&content)
        .build();

    // Show the window
    window.show();
}

/// Styles a GtkSourceView as the MIPS code view
fn style_srcview(srcview: &sourceview5::View) {
    let buffer = sourceview5::Buffer::new(None);

    if let Some(ref language) = sourceview5::LanguageManager::new().language("mal") {
        buffer.set_language(Some(language));
    }

    srcview.set_buffer(Some(&buffer));
}