use gtk::prelude::*;

pub trait Source {
    fn get_text(&self) -> String;
    fn clear(&self);
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