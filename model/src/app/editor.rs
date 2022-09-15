use std::sync::Arc;

pub struct Editor<'a> {
    text: &'a mut String,
    pc: &'a [Option<usize>],
}

impl<'a> Editor<'a> {
    pub fn new(text: &'a mut String, pc: &'a [Option<usize>]) -> Self {
        Self { text, pc }
    }
}

/*
pub const FETCH_COLOR: Color32 = Color32::from_rgb(102, 57, 49);
pub const DECODE_COLOR: Color32 = Color32::from_rgb(82, 75, 36);
pub const EXECUTE_COLOR: Color32 = Color32::from_rgb(50, 60, 57);
pub const MEMORY_COLOR: Color32 = Color32::from_rgb(63, 63, 115);
pub const WRITEBACK_COLOR: Color32 = Color32::from_rgb(69, 40, 60);
 */
