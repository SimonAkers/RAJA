use gtk::prelude::*;
use sourceview5::prelude::*;

use adw::{Application, ApplicationWindow, ColorScheme, StyleManager};
use gtk::{Builder, CssProvider, StyleContext, Widget};
use gtk::gdk::Display;

use crate::traits::*;

pub struct AdwGUI {
    adw_app: Application,
}

impl AppUI for AdwGUI {
    fn run(&self) {
        self.adw_app.run();
    }

    fn get_source(&self) -> Box<dyn Source> {
        todo!()
    }
}

impl Source for sourceview5::View {
    /// Gets the text of the GtkSourceView
    ///
    /// # Returns
    /// The text of the GtkSourceView as a String
    fn get_text(&self) -> String {
        // Get the bounds of the buffer
        let (iter1, iter2) = self.buffer().bounds();

        // Return the text within the buffer bounds
        self.buffer().text(&iter1, &iter2, false).as_str().to_owned()
    }

    /// Clears the buffer of the GtkSourceView (sets it to the empty string)
    fn clear(&self) {
        self.buffer().set_text("");
    }
}

impl AdwGUI {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id("net.shayes.raja")
            .flags(Default::default())
            .build();

        sourceview5::View::ensure_type();
        sourceview5::Buffer::ensure_type();
        sourceview5::Language::ensure_type();

        app.connect_startup(|_| AdwGUI::load_css());
        app.connect_activate(AdwGUI::build_ui);

        Self { adw_app: app }
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

    fn build_ui(app: &Application) {
        // Set the app color scheme to match the system (dark or light)
        StyleManager::default().set_color_scheme(Self::get_system_color_scheme());

        // Build UI from the specification
        let builder = Builder::from_file("src/view/res/ui/main.ui");

        // Style the source view
        let srcview: sourceview5::View = builder.object("source_view").unwrap();
        Self::style_srcview(&srcview);

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

    /// Gets a color scheme based on the system's theme (i.e. dark or light mode).
    ///
    /// # Returns
    /// A ColorScheme matching the system's theme (dark or light)
    fn get_system_color_scheme() -> ColorScheme {
        match dark_light::detect() {
            dark_light::Mode::Dark => ColorScheme::PreferDark,
            dark_light::Mode::Light => ColorScheme::PreferLight,
        }
    }

    /// Styles a GtkSourceView as the MIPS code view
    fn style_srcview(srcview: &sourceview5::View) {
        let buffer = sourceview5::Buffer::new(None);

        if let Some(ref language) = sourceview5::LanguageManager::new().language("mal") {
            buffer.set_language(Some(language));
        }

        srcview.set_buffer(Some(&buffer));
    }
}



