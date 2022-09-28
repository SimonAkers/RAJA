pub trait EditorExt {
    fn get_text(&self) -> String;
    fn clear(&self);
}

impl EditorExt for sourceview5::View {
    fn get_text(&self) -> String {
        self.get_text()
    }

    fn clear(&self) {
        self.clear();
    }
}