/// A trait defining required functionality for the application's UI.
/// !!!!! CURRENTLY UNUSED !!!!!
pub trait AppUI {
    /// Launches the application.
    fn launch();

    /// Returns the app's Source.
    fn get_source(&self) -> Box<dyn Source>;

    /// Returns the app's Console.
    fn get_console(&self) -> Box<dyn Console>;
}

/// A trait defining required functionality for a source of MIPS assembly code.
pub trait Source {
    /// Returns the text contained within the source as a String.
    fn get_text(&self) -> String;

    /// Clears the contents of the source.
    fn clear(&self);
}

/// A trait defining required functionality for a console (i.e. text I/O).
pub trait Console {
    /**
    Prints a message to the console.

    # Arguments

    - `msg` - The message to print.
     */
    fn print(&self, msg: &str);

    /**
    Prints an error message to the console.

    # Arguments

    - `msg` - The message to print.
     */
    fn print_err(&self, msg: &str);

    /**
    Prompts the user for input.

    Returns an Option containing the user's input as a &str, or None.
     */
    fn input(&self) -> Option<&str>;

    /**
    Clears the output of the console.
     */
    fn clear(&self);
}