use std::time::Duration;
use gtk::prelude::*;
use sourceview5::prelude::*;

use adw::{Application, ColorScheme, StyleManager};
use gtk::{CssProvider, StyleContext};
use gtk::gdk::Display;

use model::machine::Machine;
use util::shared::Shared;

use crate::traits::*;
use crate::app_window::AppWindow;

const APP_ID: &str = "net.shayes.raja";

pub struct AdwApp {
    app: Option<Application>,
    machine: Machine,
}

impl AdwApp {
    pub fn launch() {
        let app = Application::builder()
            .application_id(APP_ID)
            .build();

        // Create a shared instance of AdwApp
        let adw_app = Shared::new(
            Self { app: Some(app.clone()), machine: Default::default() }
        );

        // Connect the startup signal
        app.connect_startup(|_| AdwApp::load_css());

        // Connect the activate signal
        let _adw_app = adw_app.clone();
        app.connect_activate(move |app| {
            AdwApp::activate(app, &_adw_app);
        });

        // Run the app
        app.run();
    }

    fn activate(app: &Application, adw_app: &Shared<AdwApp>) {
        let window = AdwApp::build_window(app);

        // TODO: Connect UI to backend from here
        let app_ref = adw_app.borrow();

        window.btn_run().connect_clicked(|_| {
            glib::timeout_add(Duration::from_millis(100), || {
                println!("Cycle");
                Continue(true)
            });
        });

        // Show the window
        window.show();
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

    fn build_window(app: &Application) -> AppWindow {
        // Set the app color scheme to match the system (dark or light)
        StyleManager::default().set_color_scheme(Self::get_system_color_scheme());

        let window = AppWindow::new(app);

        // Style the source view
        Self::style_srcview(&window.source_view());

        window
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