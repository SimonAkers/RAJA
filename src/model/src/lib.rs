pub mod parser;
pub mod machine;
mod memory;
mod pipeline;
pub mod syscall;

pub mod callback;
mod register_file;
mod register;
mod data_types;

pub mod stages {
    pub mod writeback;
    pub use writeback::writeback;

    pub mod memory;
    pub use memory::memory;

    pub mod execute;
    pub use execute::execute;

    pub mod decode;
    pub use decode::decode;

    pub mod fetch;
    pub use fetch::fetch;

    pub mod inputs {
        pub use super::decode::IfId;
        pub use super::execute::IdEx;
        pub use super::memory::ExMem;
        pub use super::writeback::MemWb;
    }
}

pub use parser::*;
pub use machine::*;
pub use memory::*;
pub use register::*;
pub use register_file::*;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = App::default();
    eframe::start_web(canvas_id, Box::new(app))
}
