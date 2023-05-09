use std::borrow::Borrow;
use std::fs;
use std::ops::ControlFlow;
use std::time::Duration;

use adw::{Application, ColorScheme, StyleManager, Window};
use adw::gdk::pango::FontDescription;
use dark_light::Mode;
use debug_print::*;
use glib::Error;
use glib::signal::Inhibit;
use gtk::{AlertDialog, CssProvider, EventControllerKey, FileDialog, FileFilter, FontDialog, StyleContext};
use gtk::builders::FontDialogBuilder;
use gtk::gdk::{Display, Key};
use gtk::gio::{Cancellable, SimpleAction};
use gtk::pango::ffi::PANGO_SCALE;
use gtk::prelude::*;
use sourceview5::prelude::*;
use sourceview5::StyleSchemeManager;

use model::assembler;
use model::callback::Callback;
use model::machine::Machine;
use model::syscall::SyscallDiscriminants;
use util::settings::Settings;
use util::shared::Shared;

use crate::app_window::AppWindow;
use crate::ensure;
use crate::register_view::RegisterView;
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
            Self {
                app: app.clone(),
                machine: Default::default(),
            }
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

        Self::register_callbacks(adw_app.clone(), window.clone());

        // Connect build button
        Self::connect_btn_build(adw_app.clone(), window.clone());
        // Connect run button
        Self::connect_btn_run(adw_app.clone(), window.clone());

        // Connect font button
        // TODO: Move this to a "settings" window
        Self::connect_btn_settings(window.clone());

        // Connect the file buttons
        Self::connect_file_new(window.clone());
        Self::connect_file_open(window.clone());
        Self::connect_file_save_as(window.clone());

        // Connect the view buttons
        Self::connect_register_view(adw_app.clone(), window.clone());

        // Connect the "enter" key to the console
        Self::connect_console_confirm(adw_app.clone(), window.clone());

        // Show the window
        window.present();
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
                        _window.main_view().console().print(&*format!("{}", message));
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
                        _window.main_view().console().print_err(&*format!("[ERROR] {}", message));
                        debug_println!("[CONSOLE] [ERROR] {}", message);
                    }
                }
            }))
        );

        // ReadAny
        // TODO: Make read syscalls more generic
        let _window = window.clone();
        callbacks.insert(
            SyscallDiscriminants::ReadAny,
            Callback::new(Box::new(move |_| {
                _window.main_view().console().start_user_input();
            }))
        );

        let _window = window.clone();
        callbacks.insert(
            SyscallDiscriminants::Quit,
            Callback::new(Box::new(move |_| {
                let console = _window.main_view().console();
                console.print_success("[SUCCESS] Process exited");
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
            window.main_view().console().clear();
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
            window.main_view().console().clear();

            // Reset and flash the assembly to the machine
            Self::reset_flash_machine(&adw_app, &window);

            Self::start_simulator(adw_app.clone(), window.clone());
        });
    }

    fn connect_btn_settings(window: AppWindow) {
        window.btn_settings().connect_clicked(move |_| {
            let window = window.clone();

            FontDialog::new().choose_font(Some(&window.clone()), None, Cancellable::NONE, move |font| {
                match font {
                    Ok(desc) => {
                        match Self::change_font(desc) {
                            Ok(_) => {
                                AlertDialog::builder()
                                    .message("Success!")
                                    .detail("Font changed, restart to apply the changes.")
                                    .buttons(["Ok"])
                                    .build()
                                    .show(Some(&window));
                            }
                            Err(err) => {
                                AlertDialog::builder()
                                    .message("ERROR")
                                    .detail(format!("An error occurred while trying to save changes:\n{err}"))
                                    .buttons(["Ok"])
                                    .build()
                                    .show(Some(&window));
                            }
                        }
                    }

                    Err(_) => {}
                }
            })
        });
    }

    fn connect_simple_action<F>(window: AppWindow, action_name: &str, f: F) where
        F: Fn(&SimpleAction, Option<&glib::Variant>) + 'static
    {
        let action = SimpleAction::new(action_name, None);
        action.connect_activate(f);
        window.add_action(&action);
    }

    fn connect_file_new(window: AppWindow) {
        Self::connect_simple_action(window.clone(), "file-new", move |_, _| {
            let alert = AlertDialog::builder()
                .message("WARNING")
                .detail("This will erase everything in the editor!\nAre you sure you want to continue?")
                .buttons(["Yes", "No"])
                .build();

            let _window = window.clone();
            alert.choose(Some(&window), Cancellable::NONE, move |result| {
                match result {
                    Ok(index) => {
                        // If "yes" button was clicked
                        if index == 0 {
                            _window.main_view().source_view().clear();
                            _window.main_view().console().clear();
                        }
                    }
                    Err(_) => ()
                }
            });
        });
    }

    fn connect_file_open(window: AppWindow) {
        Self::connect_simple_action(window.clone(), "file-open", move |_, _| {
            // TODO: Investigate why this filter does not work
            let filter = FileFilter::new();
            filter.add_pattern("*.s");
            filter.add_pattern("*.asm");
            let dialog = FileDialog::builder()
                .title("Open File")
                .default_filter(&filter)
                .build();

            let _window = window.clone();
            dialog.open(Some(&window), Cancellable::NONE, move |result| {
                // Get the file
                let file = match result {
                    Ok(file) => file,
                    Err(_) => return
                };

                // Get the path of the file
                let path = match file.path() {
                    Some(path) => path,
                    None => return
                };

                // Read the file into the editor
                match fs::read_to_string(path) {
                    Ok(contents) => {
                        _window.main_view().source_view().set_text(contents);
                    }
                    Err(_) => {}
                }
            })
        });
    }

    fn connect_file_save_as(window: AppWindow) {
        Self::connect_simple_action(window.clone(), "file-save-as", move |_, _| {
            let dialog = FileDialog::builder()
                .title("Save As")
                .build();

            let _window = window.clone();
            dialog.save(Some(&window), Cancellable::NONE, move |result| {
                // Get the file
                let file = match result {
                    Ok(file) => file,
                    Err(_) => return
                };

                // Get the path of the file
                let path = match file.path() {
                    Some(path) => path,
                    None => return
                };

                // Get the contents to write
                let contents = _window.main_view().source_view().text();

                // Write to the file
                match fs::write(path, contents) {
                    Ok(_) => {}
                    Err(err) => {
                        // Alert the user if failed
                        AlertDialog::builder()
                            .message("ERROR: Failed to save")
                            .detail(err.to_string())
                            .buttons(["Ok"])
                            .build()
                            .show(Some(&_window));
                    }
                }
            })
        });
    }

    fn connect_register_view(adw_app: Shared<AdwApp>, window: AppWindow) {
        let machine = &mut adw_app.borrow_mut().machine;
        window.register_view().init(machine.register_file());

        Self::connect_simple_action(window.clone(), "register", move |_, _| {
            let view = window.register_view();
            view.set_visible(!view.get_visible());
        });
    }

    fn connect_console_confirm(adw_app: Shared<AdwApp>, window: AppWindow) {
        let controller = EventControllerKey::new();

        let _window = window.clone();
        controller.connect_key_pressed(move |keyval, keycode, state, _| {
            if keycode == Key::Return {
                let mut console = _window.main_view().console();

                if console.user_input_started() {
                    let machine = &mut adw_app.borrow_mut().machine;

                    // End user input
                    console.end_user_input();
                    console.print("\n");

                    // Pass input to the machine
                    machine.set_input(Some(console.input()));

                    // Continue the simulator
                    Self::start_simulator(adw_app.clone(), _window.clone());
                }
            }

            // Do not inhibit other callbacks registered to key events
            Inhibit(false)
        });

        window.main_view().console().add_controller(controller);
    }

    fn change_font(desc: FontDescription) -> std::io::Result<()> {
        let family = desc.family().unwrap_or_default().to_string();

        let size = match desc.is_size_absolute() {
            true => desc.size(),
            false => desc.size() / PANGO_SCALE,
        };

        let css = format!(".monospace {{ font: {size}pt \"{family}\" }}");

        fs::write("src/view/res/mono_style.css", css)?;
        Settings::load().set_mono_font(desc.to_string()).save()
    }

    pub fn start_simulator(adw_app: Shared<AdwApp>, window: AppWindow) {
        glib::timeout_add_local(Duration::from_millis(1), move || {
            let machine = &mut adw_app.borrow_mut().machine;

            // Cycle the machine
            let flow = machine.cycle();

            window.register_view().update(machine.register_file());

            match flow {
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
        let mut src = window.main_view().source_view().text();

        // Reset the machine
        machine.hard_reset();

        // Flash the machine
        match assembler(src) {
            Ok((mem, lbl)) => machine.flash(mem, lbl),
            Err(err) => window.main_view().console().print_err(&format!("{err}"))
        };
    }

    /// Loads the CSS for the GUI.
    fn load_css() {
        // Load the CSS file and add it to the provider
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("../res/mono_style.css"));

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
        Self::style_srcview(&window.main_view().source_view());

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
    fn text(&self) -> String {
        // Get the bounds of the buffer
        let (iter1, iter2) = self.buffer().bounds();

        // Return the text within the buffer bounds
        self.buffer().text(&iter1, &iter2, false).as_str().to_owned()
    }

    fn set_text(&self, text: String) {
        self.buffer().set_text(&text);
    }

    fn clear(&self) {
        self.buffer().set_text("");
    }
}