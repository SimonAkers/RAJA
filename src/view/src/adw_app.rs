use std::borrow::Borrow;
use std::ops::ControlFlow;
use std::time::Duration;
use debug_print::*;

use gtk::prelude::*;
use sourceview5::prelude::*;

use adw::{Application, ColorScheme, StyleManager};
use gtk::{CssProvider, StyleContext};
use gtk::gdk::Display;
use model::assembler;

use model::machine::Machine;
use model::syscall::Syscall;

use util::shared::Shared;

use crate::ensure;
use crate::traits::*;
use crate::app_window::AppWindow;

const APP_ID: &str = "net.shayes.raja";

pub struct AdwApp {
    app: Application,
    machine: Machine,
}

impl AdwApp {
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

    fn activate(app: &Application, adw_app: &Shared<AdwApp>) {
        let window = AdwApp::build_window(app);

        // TODO: Connect UI to backend from here

        // Connect build button
        Self::connect_btn_build(adw_app.clone(), window.clone());

        // Connect run button
        Self::connect_btn_run(adw_app.clone(), window.clone());

        // Show the window
        window.show();
    }

    fn connect_btn_build(adw_app: Shared<AdwApp>, window: AppWindow) {
        window.btn_build().connect_clicked(move |_| {
            debug_println!("BUILD BUTTON PRESSED");
            window.console().clear();
            Self::reset_flash_machine(&adw_app, &window);
        });
    }

    fn connect_btn_run(adw_app: Shared<AdwApp>, window: AppWindow) {
        window.btn_run().connect_clicked(move |_| {
            debug_println!("[DEBUG] Assembling and running...");

            // Clear the console
            window.console().clear();

            // Reset and flash the assembly to the machine
            Self::reset_flash_machine(&adw_app, &window);

            let adw_app = adw_app.clone();
            let window = window.clone();
            glib::timeout_add_local(Duration::from_millis(1), move || {
                let machine = &mut adw_app.borrow_mut().machine;

                // ===== BEGIN PRINT SYSCALL HANDLING =====
                // TODO: Move print syscall code out of UI code!!!!!!!!!!!
                // TODO: Optimize this, since it may be slowing down simulation
                let mut print = String::new();

                if machine.pending_syscall() {
                    machine.handle_syscall(|syscall| match syscall {
                        Syscall::Print(out) => ControlFlow::Break(print.push_str(&out)),
                        Syscall::Error(out) => ControlFlow::Break({
                            print.push_str(&format!("ERROR: {out}\n"));
                        }),
                        Syscall::Quit => ControlFlow::Break(()),
                        _ => ControlFlow::Continue(()),
                    });
                }

                if print.len() > 0 {
                    window.console().print(&*format!("{}", print));
                    debug_println!("[CONSOLE] {}", print);
                }

                // VERY BAD AND HACKY WAY TO EXIT!!!!!!
                // TODO: Exit if hit kernel but do not do it this way
                if print == "ERROR: program finished (ran into kernel)\n".to_string() {
                    return Continue(false);
                }
                // ===== END PRINT SYSCALL HANDLING =====

                // Cycle the machine
                match machine.cycle() {
                    Ok(_) => Continue(true),
                    Err(_) => { debug_println!("CYCLE FAILED"); Continue(false) },
                }
            });
        });
    }

    fn reset_flash_machine(adw_app: &Shared<AdwApp>, window: &AppWindow) {
        let machine = &mut adw_app.borrow_mut().machine;

        // Get the assembly code
        let mut src = window.source_view().get_text();
        src.push('\n');

        // Reset the machine
        machine.hard_reset();

        // Flash the machine
        match assembler(src.as_str()) {
            Ok((mem, lbl)) => machine.flash(mem, lbl),
            Err(err) => window.console().print_err(&format!("{err}"))
        };
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