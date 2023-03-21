use std::borrow::Borrow;
use std::ops::ControlFlow;
use std::time::Duration;

use adw::{Application, ColorScheme, StyleManager};
use dark_light::Mode;
use debug_print::*;
use glib::signal::Inhibit;
use gtk::{CssProvider, EventControllerKey, StyleContext};
use gtk::gdk::{Display, Key};
use gtk::prelude::*;
use sourceview5::StyleSchemeManager;
use sourceview5::prelude::*;

use model::assembler;
use model::callback::Callback;
use model::machine::Machine;
use model::syscall::SyscallDiscriminants;
use util::shared::Shared;

use crate::app_window::AppWindow;
use crate::ensure;
use crate::traits::*;

/// The application's ID
const APP_ID: &str = "net.shayes.raja";

/// A struct representing the application with a GTK/Adwaita GUI.
pub struct AdwApp {
    app: Application,
    machine: Machine,
}

impl AdwApp {
    /// Launches the application.
    pub fn launch() {
        // Ensure custom widgets are known to GTK
        ensure::ensure_types();

        let app = Application::builder()
            .application_id(APP_ID)
            .build();

        // Create a shared instance of AdwApp
        let adw_app = Shared::new(
            Self { app: app.clone(), machine: Default::default() }
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

    /**
    Called when the GUI is activated in order to connect the GUI to the backend.

    # Arguments
    - `app` - A borrowed reference to the application associated with the GUI.
    - `adw_app` - A borrowed reference to a shared instance of AdwApp.
     */
    fn activate(app: &Application, adw_app: &Shared<AdwApp>) {
        let window = AdwApp::build_window(app);

        // TODO: Connect UI to backend from here

        Self::register_callbacks(adw_app.clone(), window.clone());

        // Connect build button
        Self::connect_btn_build(adw_app.clone(), window.clone());

        // Connect run button
        Self::connect_btn_run(adw_app.clone(), window.clone());

        // Connect the "enter" key to the console
        Self::connect_console_confirm(adw_app.clone(), window.clone());

        // Show the window
        window.show();
    }

    /**
    Registers functions to be called when the simulator handles system calls.

    # Arguments
    - `adw_app` - A reference to a shared instance of AdwApp.
    - `window` - A reference to the app's window.
     */
    fn register_callbacks(adw_app: Shared<AdwApp>, window: AppWindow) {
        let machine = &mut adw_app.borrow_mut().machine;
        let callbacks = machine.get_callbacks();

        // Print
        let _window = window.clone();
        callbacks.insert(
            SyscallDiscriminants::Print,
            Callback::new(Box::new(move |info| {
                match info {
                    None => (),
                    Some(message) => {
                        _window.console().print(&*format!("{}", message));
                        debug_println!("[CONSOLE] {}", message);
                    }
                }
            }))
        );

        // Error
        let _window = window.clone();
        callbacks.insert(
            SyscallDiscriminants::Error,
            Callback::new(Box::new(move |info| {
                match info {
                    None => (),
                    Some(message) => {
                        _window.console().print_err(&*format!("[ERROR] {}", message));
                        debug_println!("[CONSOLE] [ERROR] {}", message);
                    }
                }
            }))
        );

        // ReadInt
        let _window = window.clone();
        callbacks.insert(
            SyscallDiscriminants::ReadInt,
            Callback::new(Box::new(move |_| {
                _window.console().start_user_input();
            }))
        );
    }

    /**
    Connects the "build" button to the simulator.

    # Arguments
    - `adw_app` - A reference to a shared instance of AdwApp.
    - `window` - A reference to the app's window.
    */
    fn connect_btn_build(adw_app: Shared<AdwApp>, window: AppWindow) {
        window.btn_build().connect_clicked(move |_| {
            debug_println!("BUILD BUTTON PRESSED");
            window.console().clear();
            Self::reset_flash_machine(&adw_app, &window);
        });
    }

    /**
    Connects the "run" button to the simulator.

    # Arguments
    - `adw_app` - A reference to a shared instance of AdwApp.
    - `window` - A reference to the app's window.
     */
    fn connect_btn_run(adw_app: Shared<AdwApp>, window: AppWindow) {
        window.btn_run().connect_clicked(move |_| {
            debug_println!("[DEBUG] Assembling and running...");

            // Clear the console
            window.console().clear();

            // Reset and flash the assembly to the machine
            Self::reset_flash_machine(&adw_app, &window);

            Self::start_simulator(adw_app.clone());
        });
    }

    fn connect_console_confirm(adw_app: Shared<AdwApp>, window: AppWindow) {
        let controller = EventControllerKey::new();

        let _window = window.clone();
        controller.connect_key_pressed(move |keyval, keycode, state, _| {
            if keycode == Key::Return {
                let mut console = _window.console();

                if console.user_input_started() {
                    let machine = &mut adw_app.borrow_mut().machine;

                    console.end_user_input();

                    machine.set_input(Some(console.input()));

                    Self::start_simulator(adw_app.clone());
                }
            }

            Inhibit(false)
        });

        window.console().add_controller(controller);
    }

    pub fn start_simulator(adw_app: Shared<AdwApp>) {
        glib::timeout_add_local(Duration::from_millis(1), move || {
            let machine = &mut adw_app.borrow_mut().machine;

            // Cycle the machine
            match machine.cycle() {
                ControlFlow::Continue(_) => Continue(true),
                ControlFlow::Break(_) => Continue(false)
            }
        });
    }

    /**
    Resets the simulator, then assembles and flashes the source assembly.

    # Arguments
    - `adw_app` - A borrowed reference to a shared instance of AdwApp.
    - `window` - A borrowed reference to the app's window.
     */
    fn reset_flash_machine(adw_app: &Shared<AdwApp>, window: &AppWindow) {
        let machine = &mut adw_app.borrow_mut().machine;

        // Get the assembly code
        let mut src = window.source_view().get_text();
        // Ensure newline to prevent assembler error
        src.push('\n');

        // Reset the machine
        machine.hard_reset();

        // Flash the machine
        match assembler(src.as_str()) {
            Ok((mem, lbl)) => machine.flash(mem, lbl),
            Err(err) => window.console().print_err(&format!("{err}"))
        };
    }

    /// Loads the CSS for the GUI.
    fn load_css() {
        // Load the CSS file and add it to the provider
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("../res/style.css"));

        // Add the provider to the default screen
        StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    /**
    Builds an instance of AppWindow, applying appropriate styles.

    # Arguments
    - `app` - A borrowed reference to the application associated with the GUI.

    Returns a new instance of AppWindow.
     */
    fn build_window(app: &Application) -> AppWindow {
        // Set the app color scheme to match the system (dark or light)
        StyleManager::default().set_color_scheme(
            match dark_light::detect() {
                Mode::Dark => ColorScheme::PreferDark,
                Mode::Light => ColorScheme::PreferLight,
            }
        );

        let window = AppWindow::new(app);

        // Style the source view
        Self::style_srcview(&window.source_view());

        window
    }

    /// Styles a GtkSourceView as the MIPS code view
    fn style_srcview(srcview: &sourceview5::View) {
        let buffer = sourceview5::Buffer::new(None);

        // Set the style scheme based on the system theme
        buffer.set_style_scheme(
            match dark_light::detect() {
                Mode::Dark => StyleSchemeManager::default().scheme("Adwaita-dark"),
                Mode::Light => StyleSchemeManager::default().scheme("Adwaita"),
            }.as_ref()
        );

        if let Some(ref language) = sourceview5::LanguageManager::new().language("mal") {
            buffer.set_language(Some(language));
        }

        srcview.set_buffer(Some(&buffer));
    }
}

/// See [Source][`crate::traits::Source`] for docs.
impl Source for sourceview5::View {
    fn get_text(&self) -> String {
        // Get the bounds of the buffer
        let (iter1, iter2) = self.buffer().bounds();

        // Return the text within the buffer bounds
        self.buffer().text(&iter1, &iter2, false).as_str().to_owned()
    }

    fn clear(&self) {
        self.buffer().set_text("");
    }
}